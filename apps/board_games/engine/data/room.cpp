#include "room.h"

#include <spdlog/spdlog.h>

#include "constants.hpp"
#include "game.h"
#include "libraries/utils/cpp/libpqxx/notify.h"

namespace data {

    Expected<void> insert_new_room(pqxx::connection& conn, RoomUUID uuid, std::string_view name) {
        SPDLOG_DEBUG("Insert new room with uuid '{}'", uuid);
        const std::string query =
            std::format("INSERT INTO {} (id, name, created_at, updated_at, is_public, is_open) VALUES ($1, $2, "
                        "NOW(), NOW(), True, True);",
                        ROOMS_TABLE);

        try {
            pqxx::work tx(conn);
            tx.exec(query, pqxx::params{uuid, name}).no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert new room: {}", e.what());
            return tl::unexpected(errors::InsertError{ROOMS_TABLE, query, e.what()});
        }
    }

    Expected<std::vector<RoomUUID>> get_playing_rooms(pqxx::connection& conn) {
        SPDLOG_DEBUG("Get list of playing rooms");
        const std::string query = std::format("SELECT id FROM {};", ROOMS_TABLE);

        try {
            pqxx::work tx(conn);
            std::vector<RoomUUID> ret;
            for (auto [id] : tx.query<RoomUUID>(query)) {
                ret.emplace_back(id);
            }
            return ret;
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to retrieve list of rooms: {}", e.what());
            return tl::unexpected(errors::SelectError{ROOMS_TABLE, query, e.what()});
        }
    }

    Expected<std::optional<Game>> find_game(pqxx::connection& conn, RoomUUID room_uuid) {
        SPDLOG_DEBUG("Return the game being played in room '{}'", room_uuid);
        const std::string query =
            std::format("SELECT id, game_type_id, state, payload FROM {} WHERE room_id = $1 LIMIT 1;", GAMES_TABLE);

        try {
            pqxx::work tx(conn);
            auto r = tx.exec(query, pqxx::params{room_uuid}).opt_row();
            if (!r) {
                return {std::nullopt};
            }

            // game_type_id is already the game_type.slug
            auto [id, game_type, state, state_data] = r->as<std::int64_t, GameType, GameState, GameStatePayload>();
            return {std::make_optional<Game>(id, room_uuid, game_type, state, std::move(state_data))};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to get game for the given room: {}", e.what());
            return tl::unexpected(errors::SelectError{GAMES_TABLE, query, e.what()});
        }
    }

    Expected<std::optional<Participant>> find_participant(pqxx::connection& conn, RoomUUID room,
                                                          ParticipantUUID participant_uuid) {
        SPDLOG_DEBUG("Return participant '{}' in room '{}'", participant_uuid, room);
        const std::string query = std::format(
            "SELECT role, player_number FROM {} WHERE id = $1 AND room_id = $2 LIMIT 1;", PARTICIPANTS_TABLE);

        try {
            pqxx::work tx(conn);

            auto r = tx.exec(query, pqxx::params{participant_uuid, room}).opt_row();
            if (!r) {
                SPDLOG_TRACE(" - participant not found");
                return {std::nullopt};
            }

            auto [role, player_number] = r->as<std::string, uint32_t>();
            Participant participant{room, participant_uuid, participant_role_from_string(role).value(), player_number};
            return {{participant}};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to get participant: {}", e.what());
            return tl::unexpected(errors::SelectError{PARTICIPANTS_TABLE, query, e.what()});
        }
    }

    Expected<Participant> add_participant(pqxx::connection& conn, RoomUUID room, ParticipantUUID participant,
                                          ParticipantRole role, std::optional<uint32_t> player_number) {
        SPDLOG_DEBUG("Insert participant '{}' into room '{}' with role '{}' and player_number '{}'", participant, room,
                     role, player_number ? std::to_string(player_number.value()) : "<None>");

        // FIXME: We can only add participants if there is already a game associated in the room. All
        //        participants are PLAYERs. We can simplify this a lot.

        std::optional<std::int64_t> game_id = std::nullopt;
        if (role == ParticipantRole::PLAYER) {
            // Check (and return) game in the room
            auto r = find_game(conn, room).and_then([&role](std::optional<Game>&& game) -> Expected<std::int64_t> {
                if (!game) {
                    SPDLOG_ERROR("There is no game associated to that room. Participant cannot be yet "
                                 "assigned the role '{}'",
                                 role);
                    return tl::unexpected(errors::LogicalError{
                        std::format("There is no game associated to that room. Participant cannot be yet "
                                    "assigned the role '{}'",
                                    role)});
                } else {
                    return {game->id};
                }
            });

            if (!r) {
                return tl::unexpected(r.error());
            }

            game_id = r.value();
            SPDLOG_TRACE(" - There is a game ({}) being played in the room", game_id.value());

            if (!player_number.has_value()) {
                // Count number of players for the game and assign next one
                auto r_count_players = data::count_players(conn, game_id.value());
                if (!r_count_players) {
                    return tl::unexpected(r_count_players.error());
                }
                player_number = r_count_players.value();
                SPDLOG_TRACE(" - New participant will be player number ({})", player_number.value());
            }
        }

        const std::string query =
            std::format("INSERT INTO {} (id, room_id, role, joined_at, game_id, player_number) VALUES ($1, $2, $3, "
                        "NOW(), $4, $5);",
                        PARTICIPANTS_TABLE);

        try {
            pqxx::work tx(conn);
            tx.exec(query, pqxx::params{participant, room, role, game_id, player_number}).no_rows();
            tx.commit();
            return {Participant{room, participant, role, static_cast<uint32_t>(player_number.value())}};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert participant: {}", e.what());
            return tl::unexpected(errors::InsertError{PARTICIPANTS_TABLE, query, e.what()});
        }
    }

    Expected<void> notify_room_update(pqxx::connection& conn, RoomUUID room) {
        auto r = utils::libpqxx::notify(conn, NOTIFICATION_CHANNEL_ROOM, room);
        if (r == 0) {
            return {};
        } else {
            return tl::unexpected(errors::DBNotificationError{
                std::format("Notification to channel '{}' with payload '{}' failed with error '{}'",
                            NOTIFICATION_CHANNEL_ROOM, room, r)});
        }
    }
} // namespace data

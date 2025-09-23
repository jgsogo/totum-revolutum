#include "room.h"

#include <spdlog/spdlog.h>

#include "constants.hpp"
#include "game.h"
#include "libraries/utils/cpp/libpqxx/notify.h"

namespace data {

    tl::expected<void, Error> insert_new_room(pqxx::connection& conn, RoomUUID uuid, std::string_view name) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Insert new room with uuid '{}'", uuid);
            tx.exec(std::format("INSERT INTO {} (id, name, created_at, updated_at, is_public, is_open) VALUES ($1, $2, "
                                "NOW(), NOW(), True, True);",
                                ROOMS_TABLE),
                    pqxx::params{uuid, name})
                .no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert new room: {}", e.what());
            return tl::unexpected(Error::InsertError);
        }
    }

    tl::expected<std::vector<RoomUUID>, Error> get_playing_rooms(pqxx::connection& conn) {
        try {
            pqxx::work tx(conn);
            std::vector<RoomUUID> ret;
            SPDLOG_DEBUG("Get list of playing room");
            for (auto [id] : tx.query<RoomUUID>(std::format("SELECT id FROM {};", ROOMS_TABLE))) {
                ret.emplace_back(id);
            }
            return ret;
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to retrieve list of rooms: {}", e.what());
            return tl::unexpected(Error::SelectError);
        }
    }

    tl::expected<std::optional<Game>, Error> find_game(pqxx::connection& conn, RoomUUID room_uuid) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Return the game being played in room '{}'", room_uuid);

            auto r =
                tx.exec(std::format("SELECT id, game_type_id, state, state_data FROM {} WHERE room_id = $1 LIMIT 1;",
                                    GAMES_TABLE),
                        pqxx::params{room_uuid})
                    .opt_row();
            if (!r) {
                return {std::nullopt};
            }

            // game_type_id is already the game_type.slug
            auto [id, game_type, state, state_data] = r->as<std::int64_t, GameType, GameState, GameStatePayload>();
            return {std::make_optional<Game>(id, room_uuid, game_type, state, std::move(state_data))};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to get game for the given room: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }

    tl::expected<std::optional<Participant>, Error> find_participant(pqxx::connection& conn, RoomUUID room,
                                                                     ParticipantUUID participant_uuid) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Return participant '{}' in room '{}'", participant_uuid, room);

            auto r = tx.exec(std::format("SELECT role, player_number FROM {} WHERE id = $1 AND room_id = $2 LIMIT 1;",
                                         PARTICIPANT_TABLE),
                             pqxx::params{participant_uuid, room})
                         .opt_row();
            if (!r) {
                SPDLOG_DEBUG(" - participant not found");
                return {std::nullopt};
            }

            auto [role, player_number] = r->as<std::string, uint32_t>();
            Participant participant{room, participant_uuid, participant_role_from_string(role).value(), player_number};
            return {{participant}};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to get participant: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }

    tl::expected<Participant, Error> add_participant(pqxx::connection& conn, RoomUUID room, ParticipantUUID participant,
                                                     ParticipantRole role, std::optional<uint32_t> player_number) {
        try {
            SPDLOG_DEBUG("Insert participant '{}' into room '{}' with role '{}' and player_number '{}'", participant,
                         room, role, player_number ? std::to_string(player_number.value()) : "<None>");

            // FIXME: We can only add participants if there is already a game associated in the room. All
            //        participants are PLAYERs. We can simplify this a lot.

            std::optional<std::int64_t> game_id = std::nullopt;
            if (role == ParticipantRole::PLAYER) {
                // Check (and return) game in the room
                auto r = find_game(conn, room)
                             .and_then([&role](std::optional<Game>&& game) -> tl::expected<std::int64_t, Error> {
                                 if (!game) {
                                     SPDLOG_ERROR("There is no game associated to that room. Participant cannot be yet "
                                                  "assigned the role '{}'",
                                                  role);
                                     return tl::unexpected(Error::DBError);
                                 } else {
                                     return {game->id};
                                 }
                             });

                if (!r.has_value()) {
                    return tl::unexpected(Error::DBError);
                }

                game_id = r.value();
                SPDLOG_DEBUG(" - There is a game ({}) being played in the room", game_id.value());

                if (!player_number.has_value()) {
                    // Count number of players for the game and assign next one
                    auto r_count_players = data::count_players(conn, game_id.value());
                    if (!r_count_players.has_value()) {
                        return tl::unexpected(Error::DBError);
                    }
                    player_number = r_count_players.value();
                    SPDLOG_DEBUG(" - New participant will be player number ({})", player_number.value());
                }
            }

            pqxx::work tx(conn);
            tx.exec(std::format(
                        "INSERT INTO {} (id, room_id, role, joined_at, game_id, player_number) VALUES ($1, $2, $3, "
                        "NOW(), $4, $5);",
                        PARTICIPANT_TABLE),
                    pqxx::params{participant, room, role, game_id, player_number})
                .no_rows();
            tx.commit();
            return {Participant{room, participant, role, static_cast<uint32_t>(player_number.value())}};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert participant: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }

    tl::expected<void, Error> notify_room_update(pqxx::connection& conn, RoomUUID room) {
        auto r = utils::libpqxx::notify(conn, NOTIFICATION_CHANNEL_ROOM, room);
        if (r == 0) {
            return {};
        } else {
            return tl::unexpected(Error::NotifcationFailed);
        }
    }
} // namespace data

#include "room.h"

#include <spdlog/spdlog.h>

#include "apps/board_games/engine/db/notify.h"

namespace data {

    static constexpr std::string_view ROOMS_TABLE = "board_games_core_room";
    static constexpr std::string_view GAMES_TABLE = "board_games_core_game";
    static constexpr std::string_view PARTICIPANT_TABLE = "board_games_core_participant";

    static constexpr std::string_view NOTIFICATION_CHANNEL_ROOM = "room_update";

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

    tl::expected<std::optional<int64_t>, Error> game_in_room(pqxx::connection& conn, RoomUUID room_uuid) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Return the game being played in room '{}'", room_uuid);

            auto r = tx.exec(std::format("SELECT id FROM {} WHERE room_id = $1 LIMIT 1;", GAMES_TABLE),
                             pqxx::params{room_uuid})
                         .opt_row();
            if (!r) {
                return {std::nullopt};
            }
            int64_t game_id = std::get<0>(r->as<int64_t>());
            return {{game_id}};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to get game for the given room: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }

    tl::expected<void, Error> add_participant(pqxx::connection& conn, RoomUUID room, ParticipantUUID participant,
                                              ParticipantRole role) {
        try {
            SPDLOG_DEBUG("Insert participant '{}' into room '{}' with role '{}'", participant, room, role);

            std::optional<int64_t> game_id = std::nullopt;
            if (role == ParticipantRole::PLAYER) {
                auto r = game_in_room(conn, room);
                if (!r.has_value()) {
                    return tl::unexpected(Error::DBError);
                }
                auto r_value = r.value();
                if (!r_value) {
                    SPDLOG_ERROR(
                        "There is no game associated to that room. Participant cannot be yet assigned the role '{}'",
                        role);
                    return tl::unexpected(Error::DBError);
                }

                game_id = r_value.value();
            }

            pqxx::work tx(conn);
            tx.exec(std::format(
                        "INSERT INTO {} (id, room_id, role, joined_at, game_id, player_number) VALUES ($1, $2, $3, "
                        "NOW(), $4, NULL);",
                        PARTICIPANT_TABLE),
                    pqxx::params{participant, room, role, game_id})
                .no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert participant: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }

    tl::expected<void, Error> notify_room_update(pqxx::connection& conn, RoomUUID room) {
        auto r = db::notify(conn, NOTIFICATION_CHANNEL_ROOM, room);
        if (r == 0) {
            return {};
        } else {
            return tl::unexpected(Error::NotifcationFailed);
        }
    }
} // namespace data

#include "game.h"

#include <spdlog/spdlog.h>

#include "room.h"

namespace data {

    // static constexpr std::string_view ROOMS_TABLE = "board_games_core_room";
    static constexpr std::string_view GAMES_TABLE = "board_games_core_game";
    static constexpr std::string_view EVENT_LOG_TABLE = "board_games_core_eventlog";
    static constexpr std::string_view GAME_ACTION_TABLE = "board_games_core_gameaction";
    static constexpr std::string_view PARTICIPANT_TABLE = "board_games_core_participant";

    tl::expected<void, Error> start_game(pqxx::connection& conn, RoomUUID uuid, GameType game) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Insert game into room '{}'", uuid);
            tx.exec(std::format("INSERT INTO {} (room_id, game_type_id, created_at, updated_at, state, state_data) "
                                "VALUES ($1, $2, NOW(), NOW(), 'waiting', '');",
                                GAMES_TABLE),
                    pqxx::params{uuid, game})
                .no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert game: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }

    tl::expected<void, Error> remove_game(pqxx::connection& conn, RoomUUID room_uuid) {
        try {
            SPDLOG_DEBUG("Remove game from room '{}'", room_uuid);

            // If the database is created using Django, the 'on_delete.CASCADE' are not propagated
            // to the database, so we need to handle them ourselves
            //  - Get the 'game' associated to the room
            // TODO: Use 'room/game_in_room' above
            SPDLOG_TRACE("Check (and retrieve) if there is any game associated to the given room");
            auto game_id_expected = game_in_room(conn, room_uuid);
            if (!game_id_expected.has_value()) {
                return tl::unexpected(Error::DBError);
            }
            if (!game_id_expected.value()) {
                SPDLOG_TRACE("There is no game associated to the given room. Nothing to do");
                return {};
            }
            int64_t game_id = game_id_expected.value().value();

            pqxx::work tx(conn);
            //  - remove 'event_log'
            SPDLOG_TRACE("Remove event_log entries associated to the game in the room");
            tx.exec(std::format("DELETE FROM {} WHERE game_id = $1;", EVENT_LOG_TABLE), pqxx::params{game_id})
                .no_rows();

            //  - remove 'game_action'
            SPDLOG_TRACE("Remove game_action entries associated to the game in the room");
            tx.exec(std::format("DELETE FROM {} WHERE game_id = $1;", GAME_ACTION_TABLE), pqxx::params{game_id})
                .no_rows();

            //  - remove 'game' from 'participants
            SPDLOG_TRACE("Clear the 'game_id' entry from the participants playing the game we are about to remove");
            tx.exec(std::format("UPDATE {} SET game_id = NULL WHERE game_id = $1;", PARTICIPANT_TABLE),
                    pqxx::params{game_id})
                .no_rows();

            // And now we can finally remove the row from the games table
            SPDLOG_TRACE("Finally remove the game from the table");
            tx.exec(std::format("DELETE FROM {} WHERE room_id = $1;", GAMES_TABLE), pqxx::params{room_uuid}).no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to remove game: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }

    tl::expected<std::uint8_t, Error> count_players(pqxx::connection& conn, std::int32_t game_id) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Count players for game '{}'", game_id);
            auto r =
                tx.exec(std::format("SELECT count(*)::int AS count FROM {} WHERE game_id = $1;", PARTICIPANT_TABLE),
                        pqxx::params{game_id})
                    .one_field();
            auto num_players = r.as<int>();
            SPDLOG_DEBUG(" - There are {} players in the game already", num_players);
            return {num_players};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to count players for game: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }
} // namespace data

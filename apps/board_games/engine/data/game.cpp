#include "game.h"

#include <fmt/ranges.h>
#include <spdlog/spdlog.h>

#include "constants.hpp"
#include "room.h"

namespace data {

    Expected<void> start_game(pqxx::connection& conn, RoomUUID uuid, GameType game,
                              const GameStatePayload& game_state_data) {
        SPDLOG_DEBUG("Insert game into room '{}'", uuid);
        const std::string query =
            std::format("INSERT INTO {} (room_id, game_type_id, created_at, updated_at, state, state_data) "
                        "VALUES ($1, $2, NOW(), NOW(), 'waiting', $3);",
                        GAMES_TABLE);

        try {
            pqxx::work tx(conn);
            tx.exec(query, pqxx::params{uuid, game, game_state_data}).no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert game: {}", e.what());
            return tl::unexpected(errors::InsertError{GAMES_TABLE, query, e.what()});
        }
    }

    Expected<void> remove_game(pqxx::connection& conn, RoomUUID room_uuid) {
        // If the database is created using Django, the 'on_delete.CASCADE' are not propagated
        // to the database, so we need to handle them ourselves
        //  - Get the 'game' associated to the room
        SPDLOG_DEBUG("Remove the game associated with the given room");
        return find_game(conn, room_uuid)
            .and_then([&conn, &room_uuid](std::optional<Game>&& game_opt) -> Expected<void> {
                return game_opt
                    .transform([&conn, &room_uuid](Game& game) -> Expected<void> {
                        try {
                            pqxx::work tx(conn);
                            //  - remove 'event_log'
                            SPDLOG_TRACE("Remove event_log entries associated to the game in the room");
                            tx.exec(std::format("DELETE FROM {} WHERE game_id = $1;", EVENT_LOG_TABLE),
                                    pqxx::params{game.id})
                                .no_rows();

                            //  - remove 'game_action'
                            SPDLOG_TRACE("Remove game_action entries associated to the game in the room");
                            tx.exec(std::format("DELETE FROM {} WHERE game_id = $1;", GAME_ACTION_TABLE),
                                    pqxx::params{game.id})
                                .no_rows();

                            //  - remove 'game' from 'participants
                            SPDLOG_TRACE("Clear the 'game_id' entry from the participants playing the game we are "
                                         "about to remove");
                            tx.exec(std::format("UPDATE {} SET game_id = NULL WHERE game_id = $1;", PARTICIPANT_TABLE),
                                    pqxx::params{game.id})
                                .no_rows();

                            // And now we can finally remove the row from the games table
                            SPDLOG_TRACE("Finally remove the game from the table");
                            tx.exec(std::format("DELETE FROM {} WHERE room_id = $1;", GAMES_TABLE),
                                    pqxx::params{room_uuid})
                                .no_rows();
                            tx.commit();
                            return {};
                        } catch (const std::exception& e) {
                            return tl::unexpected(
                                errors::DatabaseError{std::format("Failed to remove game: {}", e.what())});
                        }
                    })
                    .value_or(Expected<void>{});
            });
    }

    Expected<std::uint8_t> count_players(pqxx::connection& conn, std::int32_t game_id) {
        SPDLOG_DEBUG("Count players for game '{}'", game_id);
        const std::string query =
            std::format("SELECT count(*)::int AS count FROM {} WHERE game_id = $1;", PARTICIPANT_TABLE);

        try {
            pqxx::work tx(conn);
            auto r = tx.exec(query, pqxx::params{game_id}).one_field();
            auto num_players = r.as<int>();
            SPDLOG_TRACE(" - There are {} players in the game already", num_players);
            return {num_players};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to count players for game: {}", e.what());
            return tl::unexpected(errors::SelectError{PARTICIPANT_TABLE, query, e.what()});
        }
    }

    Expected<std::int64_t> store_action(pqxx::connection& conn, std::int32_t game_id, ParticipantUUID participant,
                                        std::string_view action_type, const GameActionPayload& payload, bool applied) {
        SPDLOG_DEBUG("Insert game_action for game '{}'", game_id);
        const std::string query =
            std::format("INSERT INTO {} (game_id, participant_id, timestamp, action_type, payload, applied) "
                        "VALUES ($1, $2, NOW(), $3, $4, $5) RETURNING id;",
                        GAME_ACTION_TABLE);

        try {
            pqxx::work tx(conn);
            auto r = tx.exec(query, pqxx::params{game_id, participant, action_type, payload, applied}).one_field();
            auto game_action_id = r.as<std::int64_t>();
            tx.commit();
            return {game_action_id};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert game_action: {}", e.what());
            return tl::unexpected(errors::InsertError{GAME_ACTION_TABLE, query, e.what()});
        }
    }

    Expected<void> store_eventlog(pqxx::connection& conn, std::int32_t game_id, std::string_view event_type,
                                  const EventLogPayload& payload, std::int64_t action_id) {
        SPDLOG_DEBUG("Insert eventlog for game '{}'", game_id);
        const std::string query = std::format("INSERT INTO {} (game_id, timestamp, event_type, payload, action_id) "
                                              "VALUES ($1, NOW(), $2, $3, $4);",
                                              EVENT_LOG_TABLE);

        try {
            pqxx::work tx(conn);
            tx.exec(query, pqxx::params{game_id, event_type, payload, action_id});
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert event_log: {}", e.what());
            return tl::unexpected(errors::InsertError{EVENT_LOG_TABLE, query, e.what()});
        }
    }

    Expected<void> update_game_state(pqxx::connection& conn, std::int32_t game_id, GameState state,
                                     const GameStatePayload& state_data) {
        SPDLOG_DEBUG("Update game state for game '{}'", game_id);
        const std::string query = std::format("UPDATE {} SET state = $2, state_data = $3 WHERE id = $1;", GAMES_TABLE);

        try {
            pqxx::work tx(conn);
            tx.exec(query, pqxx::params{game_id, state, state_data});
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to update game state: {}", e.what());
            return tl::unexpected(errors::UpdateError{GAMES_TABLE, query, e.what()});
        }
    }

    Expected<void> set_active_games(pqxx::connection& conn, const std::vector<GameType>& active_games) {
        SPDLOG_DEBUG("Set active games: {}", active_games);
        try {
            pqxx::work tx(conn);

            std::string query;
            pqxx::params values;

            if (!active_games.empty()) {
                std::ostringstream out;
                pqxx::placeholders name;
                for (auto it = active_games.begin(); it != active_games.end() - 1; it++) {
                    out << name.get() << ", ";
                    name.next();
                    values.append(*it);
                }
                out << name.get();
                name.next();
                values.append(active_games.back());

                query = out.str();
            }

            tx.exec(std::format("UPDATE {} SET enabled = CASE "
                                "   WHEN slug IN ({}) THEN true "
                                "   ELSE false "
                                "END;",
                                GAME_TYPE_TABLE, query),
                    values);
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to set enabled games: {}", e.what());
            return tl::unexpected(errors::DatabaseError{std::format("Failed to set enabled games: {}", e.what())});
        }
    }

} // namespace data

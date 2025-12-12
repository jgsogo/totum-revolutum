#pragma once

#include <expected>
#include <pqxx/pqxx>

#include "apps/board_games/engine/errors/errors.hpp"

#include "models/game_state.hpp"
#include "models/game_type.hpp"
#include "models/payload.hpp"
#include "models/uuid_participant.hpp"
#include "models/uuid_room.hpp"

namespace data {

    // Starts a new game in the given room. If there was a game already playing, it will fail
    Expected<void> start_game(pqxx::connection& conn, RoomUUID uuid, GameType game, const GamePayload& game_state_data);

    // Removes a game, if it exists, from the given room
    Expected<void> remove_game(pqxx::connection& conn, RoomUUID uuid);

    Expected<std::uint8_t> count_players(pqxx::connection& conn, std::int32_t game_id);

    Expected<std::int64_t> store_action(pqxx::connection& conn, std::int32_t game_id, ParticipantUUID participant,
                                        std::string_view action_type, const ActionPayload& payload, bool applied);

    Expected<void> store_eventlog(pqxx::connection& conn, std::int32_t game_id, std::string_view event_type,
                                  const EventPayload& payload, std::int64_t action_id);

    Expected<void> update_game_state(pqxx::connection& conn, std::int32_t game_id, GameState state,
                                     const GamePayload& state_data);

    Expected<void> set_active_games(pqxx::connection& conn, const std::vector<GameType>& active_games);
} // namespace data

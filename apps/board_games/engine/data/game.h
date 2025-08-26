#pragma once

#include <expected>
#include <pqxx/pqxx>

#include "tl/expected.hpp"

#include "errors.h"
#include "models/game_state.hpp"
#include "models/game_type.hpp"
#include "models/payload.hpp"
#include "models/uuid_participant.hpp"
#include "models/uuid_room.hpp"

namespace data {

    // Starts a new game in the given room. If there was a game already playing, it will fail
    tl::expected<void, Error> start_game(pqxx::connection& conn, RoomUUID uuid, GameType game,
                                         const GameStatePayload& game_state_data);

    // Removes a game, if it exists, from the given room
    tl::expected<void, Error> remove_game(pqxx::connection& conn, RoomUUID uuid);

    tl::expected<std::uint8_t, Error> count_players(pqxx::connection& conn, std::int32_t game_id);

    tl::expected<std::int64_t, Error> store_action(pqxx::connection& conn, std::int32_t game_id,
                                                   ParticipantUUID participant, std::string_view action_type,
                                                   const GameActionPayload& payload, bool applied);

    tl::expected<void, Error> store_eventlog(pqxx::connection& conn, std::int32_t game_id, std::string_view event_type,
                                             const EventLogPayload& payload, std::int64_t action_id);

    tl::expected<void, Error> update_game_state(pqxx::connection& conn, std::int32_t game_id, GameState state,
                                                const GameStatePayload& state_data);

    tl::expected<void, Error> set_active_games(pqxx::connection& conn, const std::vector<GameType>& active_games);
} // namespace data

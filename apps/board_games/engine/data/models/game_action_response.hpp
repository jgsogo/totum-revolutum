#pragma once

#include <string>

#include "game_state.hpp"
#include "payload.hpp"

namespace data {

    struct GameActionResponse {
        GameActionResponse(std::string_view action_type, std::string_view eventlog_type,
                           EventPayload&& eventlog_payload, GamePayload&& new_game_state_data, GameState new_game_state)
            : action_type{action_type}, eventlog_type{eventlog_type}, eventlog_payload{std::move(eventlog_payload)},
              new_game_state_data{std::move(new_game_state_data)}, new_game_state{new_game_state} {}

        std::string action_type;
        std::string eventlog_type;
        EventPayload eventlog_payload;
        GamePayload new_game_state_data;
        GameState new_game_state;
    };

} // namespace data

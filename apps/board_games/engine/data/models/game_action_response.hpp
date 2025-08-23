#pragma once

#include <string>

#include "game_state.hpp"
#include "payload.hpp"

namespace data {

    struct GameActionResponse {
        std::string action_type;
        std::string eventlog_type;
        EventLogPayload eventlog_payload;
        GameStatePayload new_game_state_data;
        GameState new_game_state;
    };

} // namespace data

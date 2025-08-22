#pragma once

#include <string>

#include "game_state.hpp"

namespace data {

    struct GameActionResponse {
        std::string action_type;
        std::string eventlog_type;
        std::string eventlog_payload;
        std::string new_game_state_data;
        GameState new_game_state;
    };

} // namespace data

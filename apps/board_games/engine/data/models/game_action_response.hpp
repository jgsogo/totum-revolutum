#pragma once

#include <string>
#include <vector>

#include "game_state.hpp"
#include "payload.hpp"

namespace data {

    struct GameActionResponse {
        std::string_view action_type;
        std::vector<std::pair<std::string_view, EventPayload>> events;
        GamePayload new_game_payload;
        GameState new_game_state;
    };

} // namespace data

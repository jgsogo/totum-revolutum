#pragma once

#include <string>
#include <vector>

#include "game_state.hpp"
#include "payload.hpp"

#include "game_action_response.hpp"

namespace data {

    struct GameJoinResponse {
        uint8_t player_number;
        GameActionResponse action_response;
    };

} // namespace data

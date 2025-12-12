#pragma once

#include <string>

#include "game_state.hpp"
#include "payload.hpp"

namespace data {

    struct GameActionResponse {
        GameActionResponse(std::string_view action_type, std::string_view event_type, EventPayload&& event_payload,
                           GamePayload&& new_game_payload, GameState new_game_state)
            : action_type{action_type}, event_type{event_type}, event_payload{std::move(event_payload)},
              new_game_payload{std::move(new_game_payload)}, new_game_state{new_game_state} {}

        std::string action_type;
        std::string event_type;
        EventPayload event_payload;
        GamePayload new_game_payload;
        GameState new_game_state;
    };

} // namespace data

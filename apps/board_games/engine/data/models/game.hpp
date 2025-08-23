#pragma once

#include "game_state.hpp"
#include "game_type.hpp"
#include "uuid_room.hpp"
#include "payload.hpp"

namespace data {
    struct Game {
        std::int64_t id;
        RoomUUID room;
        GameType type;
        GameState state;
        GameStatePayload state_data;
    };
} // namespace data

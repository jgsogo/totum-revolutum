#pragma once

#include "game_state.hpp"
#include "game_type.hpp"
#include "uuid_room.hpp"

namespace data {
    struct Game {
        std::int64_t id;
        RoomUUID room;
        GameType type;
        GameState state;
        std::string state_data;
    };
} // namespace data

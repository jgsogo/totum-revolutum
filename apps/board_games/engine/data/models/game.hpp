#pragma once

#include "game_state.hpp"
#include "game_type.hpp"
#include "payload.hpp"
#include "uuid_room.hpp"

namespace data {
    struct Game {
        Game() = delete;
        Game(const Game&) = delete;
        explicit Game(Game&&) = default;
        explicit Game(std::int64_t id, RoomUUID room, GameType type, GameState state, GameStatePayload&& payload)
            : id{id}, room{room}, type{type}, state{state}, state_data{std::move(payload)} {}

        std::int64_t id;
        RoomUUID room;
        GameType type;
        GameState state;
        GameStatePayload state_data;
    };
} // namespace data

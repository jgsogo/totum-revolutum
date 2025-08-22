#pragma once

#include <expected>
#include <pqxx/pqxx>

#include "tl/expected.hpp"

#include "errors.h"
#include "models/game_type.hpp"
#include "models/uuid_room.hpp"

namespace data {

    // Starts a new game in the given room. If there was a game already playing, it will fail
    tl::expected<void, Error> start_game(pqxx::connection& conn, RoomUUID uuid, GameType game);

    // Removes a game, if it exists, from the given room
    tl::expected<void, Error> remove_game(pqxx::connection& conn, RoomUUID uuid);

    tl::expected<std::uint8_t, Error> count_players(pqxx::connection& conn, std::int32_t game_id);

} // namespace data

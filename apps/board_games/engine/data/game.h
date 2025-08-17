#pragma once

#include <expected>
#include <pqxx/pqxx>

#include "tl/expected.hpp"

#include "errors.h"
#include "game_type.hpp"
#include "uuid.hpp"

namespace data {

    // Starts a new game in the given room. If there was a game already playing, it will fail
    tl::expected<void, Error> start_game(pqxx::connection& conn, RoomUUID uuid, GameType game);

    // Removes a game, if it exists, from the given room
    tl::expected<void, Error> remove_game(pqxx::connection& conn, RoomUUID uuid);

} // namespace data

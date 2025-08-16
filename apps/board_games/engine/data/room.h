#pragma once

#include <expected>
#include <pqxx/pqxx>
#include <vector>

#include "tl/expected.hpp"

#include "game_type.hpp"
#include "uuid.hpp"

namespace data {

    // FIXME: Use strong types instead, see https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjhxti2/,
    //        and we can write some widening-variant like https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjrfl5z/
    //        and move this to a reusable taget built on top of std::expected
    enum class Error {
        InsertError,
        SelectError,
        DBError,
    };

    [[maybe_unused]] tl::expected<void, Error> insert_new_room(pqxx::connection& conn, RoomUUID uuid,
                                                               std::string_view name);

    [[maybe_unused]] tl::expected<std::vector<RoomUUID>, Error> get_playing_rooms(pqxx::connection& conn);

    // Starts a new game in the given room. If there was a game already playing, it will fail
    tl::expected<void, Error> start_game(pqxx::connection& conn, RoomUUID uuid, GameType game);

    // Removes a game, if it exists, from the given room
    tl::expected<void, Error> remove_game(pqxx::connection& conn, RoomUUID uuid);

} // namespace data

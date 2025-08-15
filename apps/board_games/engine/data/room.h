#pragma once

#include "uuid.hpp"
#include <expected>
#include <pqxx/pqxx>
#include <vector>

namespace data {

    // FIXME: Use strong types instead, see https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjhxti2/,
    //        and we can write some widening-variant like https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjrfl5z/
    //        and move this to a reusable taget built on top of std::expected
    enum class Error {
        InsertError,
        SelectError,
    };

    [[maybe_unused]] std::expected<void, Error> insert_new_room(pqxx::connection& conn, RoomUUID uuid,
                                                                std::string_view name);

    [[maybe_unused]] std::expected<std::vector<RoomUUID>, Error> get_playing_rooms(pqxx::connection& conn);

    // Starts a new game in the given room. If there was a game already playing, it will be obliterated and
    // substituted by the new one.
    std::expected<void, Error> start_game(pqxx::connection& conn, RoomUUID uuid,
                                          std::string game /* FIXME: Use strong type for game_type */);

} // namespace data

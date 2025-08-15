#pragma once

#include "uuid.hpp"
#include <expected>
#include <pqxx/pqxx>
#include <vector>

namespace data {

    enum class Error {
        InsertError,
        SelectError,
    };

    [[maybe_unused]] std::expected<void, Error> insert_new_room(pqxx::connection& conn, RoomUUID uuid,
                                                                std::string_view name);

    [[maybe_unused]] std::expected<std::vector<RoomUUID>, Error> get_playing_rooms(pqxx::connection& conn);

} // namespace data

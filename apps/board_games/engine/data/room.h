#pragma once

#include <expected>
#include <pqxx/pqxx>
#include <vector>

#include "tl/expected.hpp"

#include "errors.h"
#include "game_type.hpp"
#include "uuid.hpp"

namespace data {

    [[maybe_unused]] tl::expected<void, Error> insert_new_room(pqxx::connection& conn, RoomUUID uuid,
                                                               std::string_view name);

    [[maybe_unused]] tl::expected<std::vector<RoomUUID>, Error> get_playing_rooms(pqxx::connection& conn);

} // namespace data

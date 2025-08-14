#pragma once

#include <pqxx/pqxx>
#include <vector>

namespace db {
    std::vector<std::string> get_playing_room_ids(pqxx::connection& conn);
}

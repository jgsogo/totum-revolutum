#pragma once

#include <pqxx/pqxx>
#include <string_view>

namespace utils::libpqxx {

    // Sends a NOTIFY call to the given channel with the given payload
    int notify(pqxx::connection& conn, std::string_view channel, std::string_view payload);

    // Sends a NOTIFY call to the given channel
    int notify(pqxx::connection& conn, std::string_view channel);

} // namespace utils::libpqxx

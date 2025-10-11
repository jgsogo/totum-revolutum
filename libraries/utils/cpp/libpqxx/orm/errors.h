
#pragma once

#include <spdlog/spdlog.h>

namespace utils::db {

    // struct ErrorNotFound {};
    struct DatabaseError {};
    // struct ErrorMultipleFound {};
} // namespace utils::db

// Required for spdlog
template <> struct fmt::formatter<utils::db::DatabaseError> : fmt::formatter<std::string> {
    auto format(utils::db::DatabaseError p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "DatabaseError");
    }
};

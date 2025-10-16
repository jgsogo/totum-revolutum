
#pragma once

#include <spdlog/spdlog.h>

namespace utils::qt::models {

    struct ErrorItemNotFound {};
} // namespace utils::qt::models

// // Required for spdlog
template <> struct fmt::formatter<utils::qt::models::ErrorItemNotFound> : fmt::formatter<std::string> {
    auto format(utils::qt::models::ErrorItemNotFound p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "ErrorItemNotFound");
    }
};

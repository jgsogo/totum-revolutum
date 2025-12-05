#pragma once

#include <format>
#include <string>

#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/string_literal.hpp"

namespace utils {

    namespace errors {
        template <StringLiteral error_id> struct BaseError {
            constexpr static auto error_identifier = error_id;
            std::string msg;
        };

    } // namespace errors

    using NotImplemented = errors::BaseError<"NotImplemented">;
} // namespace utils

// Required for spdlog
template <utils::StringLiteral T> struct fmt::formatter<utils::errors::BaseError<T>> : fmt::formatter<std::string> {
    auto format(utils::errors::BaseError<T> p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "{}: {}", p.error_identifier, p.msg);
    }
};

// Required for std::format
template <utils::StringLiteral T> struct std::formatter<utils::errors::BaseError<T>> : std::formatter<std::string> {
    auto format(const utils::errors::BaseError<T>& p, std::format_context& ctx) const {
        return std::formatter<std::string>::format(std::format("{}: {}", p.error_identifier, p.msg), ctx);
    }
};

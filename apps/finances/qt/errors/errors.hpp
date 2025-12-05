#pragma once

#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/expected_type/expected_type.hpp"

namespace error {

    struct Error {
        Error(std::string msg_) : msg{msg_} {};

        std::string msg;
    };

    struct InputFieldNotSet : public Error {
        InputFieldNotSet(std::string field_name) : Error(std::format("Input field '{}' not set", field_name)) {};
    };

} // namespace error

// Required for std::format
template <> struct std::formatter<error::Error> : std::formatter<std::string> {
    auto format(error::Error e, format_context& ctx) const { return formatter<string>::format(e.msg, ctx); }
};

// Required for spdlog
template <> struct fmt::formatter<error::Error> : fmt::formatter<std::string> {
    auto format(error::Error e, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "{}", e.msg);
    }
};

template <typename T> using ExpectedType = utils::ExpectedType<T, error::Error>;

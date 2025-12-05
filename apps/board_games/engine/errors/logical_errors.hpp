#pragma once

#include "libraries/utils/cpp/expected_type.hpp"

namespace errors {

    //! Errors raised because some invariant was not fulfilled.
    //!
    //! It should be possible to avoid these errors by modifying the application logic
    using LogicalError = utils::errors::BaseError<class LogicalErrorTag>;

} // namespace errors

template <> struct fmt::formatter<errors::LogicalError> : fmt::formatter<std::string> {
    auto format(errors::LogicalError p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "LogicalError: {}", p.msg);
    }
};

template <> struct std::formatter<errors::LogicalError> : std::formatter<std::string> {
    auto format(const errors::LogicalError& p, std::format_context& ctx) const {
        return std::formatter<std::string>::format(std::format("LogicalError: {}", p.msg), ctx);
    }
};

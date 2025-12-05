#pragma once

#include <variant>

#include "./errors.hpp"

namespace utils {

    template <typename... Errs> struct ErrorType : std::variant<NotImplemented, Errs...> {
        using std::variant<NotImplemented, Errs...>::variant;
    };

} // namespace utils

// Required for spdlog
template <typename... Args> struct fmt::formatter<utils::ErrorType<Args...>> : fmt::formatter<std::string> {
    auto format(utils::ErrorType<Args...> p, format_context& ctx) const -> decltype(ctx.out()) {
        return std::visit(
            [&ctx](auto&& arg) {
                // using T = std::decay_t<decltype(arg)>;
                return fmt::format_to(ctx.out(), "{}", arg);
            },
            p);
    }
};

// Required for std::format
template <typename... Args> struct std::formatter<utils::ErrorType<Args...>> : std::formatter<std::string> {
    auto format(const utils::ErrorType<Args...>& p, std::format_context& ctx) const {
        return std::visit(
            [&ctx, this](auto&& arg) {
                // using T = std::decay_t<decltype(arg)>;
                return this->formatter<std::string>::format(std::format("{}", arg), ctx);
            },
            p);
    }
};

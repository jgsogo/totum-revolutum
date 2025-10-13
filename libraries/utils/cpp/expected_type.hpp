
#pragma once

#include <string>
#include <variant>

#include <spdlog/spdlog.h>
#include <tl/expected.hpp>

#include "libraries/utils/cpp/concepts/alternative_c.hpp"

// Main ideas taken from https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjhxti2/

namespace utils {

    struct NotImplemented {
        std::string msg;
    };

    template <typename T, typename... Errs>
    struct ExpectedType : tl::expected<T, std::variant<NotImplemented, Errs...>> {
        using tl::expected<T, std::variant<NotImplemented, Errs...>>::expected;

        // /// A constructor for the NotImplemented error type
        // explicit ExpectedType(NotImplemented&& e) : tl::expected<T, std::variant<NotImplemented,
        // Errs...>>{tl::make_unexpected(std::move(e))} {}

        /// A constructor that can take any of the unexpected types
        template <typename Err>
            requires utils::concepts::alternative_in_pack<Err, NotImplemented, Errs...>
        explicit ExpectedType(Err&& e)
            : tl::expected<T, std::variant<NotImplemented, Errs...>>{tl::make_unexpected(std::move(e))} {}

        /// Cast-move operator to convert to an `ExpectedType` with a superset of error types
        template <class... ToArgs> operator ExpectedType<T, ToArgs...>() && {
            if (this->has_value()) {
                return {std::move(this->value())};
            } else {
                // std::variant<NotImplemented, Errs...> err = std::move(this->error());
                return std::visit(
                    [](auto&& arg) {
                        // using TErr = std::decay_t<decltype(arg)>;
                        return ExpectedType<T, ToArgs...>(std::move(arg));
                    },
                    std::move(this->error()));
            }
        }
    };

} // namespace utils

// Required for spdlog
template <typename... Args> struct fmt::formatter<std::variant<Args...>> : fmt::formatter<std::string> {
    auto format(std::variant<Args...> p, format_context& ctx) const -> decltype(ctx.out()) {
        return std::visit(
            [&ctx](auto&& arg) {
                // using T = std::decay_t<decltype(arg)>;
                return fmt::format_to(ctx.out(), "{}", arg);
            },
            p);
    }
};

template <> struct fmt::formatter<utils::NotImplemented> : fmt::formatter<std::string> {
    auto format(utils::NotImplemented p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "NotImplemented: {}", p.msg);
    }
};

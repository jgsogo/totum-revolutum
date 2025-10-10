
#pragma once

#include <tl/expected.hpp>
#include <variant>

// Main ideas taken from https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjhxti2/

namespace utils::db {

    template <typename T, typename... Errs> using Expected = tl::expected<T, std::variant<Errs...>>;

    struct ErrorNotFound {};
    struct DatabaseError {};
    struct ErrorMultipleFound {};
} // namespace utils::db

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

template <> struct fmt::formatter<utils::db::DatabaseError> : fmt::formatter<std::string> {
    auto format(utils::db::DatabaseError p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "DatabaseError");
    }
};

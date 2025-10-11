
#pragma once

#include <tl/expected.hpp>
#include <variant>

// Main ideas taken from https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjhxti2/

namespace utils::qt::models {

    template <typename T, typename... Errs> using Expected = tl::expected<T, std::variant<Errs...>>;

    struct ErrorItemNotFound {};
} // namespace utils::qt::models

// // Required for spdlog
// template <typename... Args> struct fmt::formatter<std::variant<Args...>> : fmt::formatter<std::string> {
//     auto format(std::variant<Args...> p, format_context& ctx) const -> decltype(ctx.out()) {
//         return std::visit(
//             [&ctx](auto&& arg) {
//                 // using T = std::decay_t<decltype(arg)>;
//                 return fmt::format_to(ctx.out(), "{}", arg);
//             },
//             p);
//     }
// };

template <> struct fmt::formatter<utils::qt::models::ErrorItemNotFound> : fmt::formatter<std::string> {
    auto format(utils::qt::models::ErrorItemNotFound p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "ErrorItemNotFound");
    }
};

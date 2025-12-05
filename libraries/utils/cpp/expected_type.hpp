
#pragma once

#include <expected>
#include <format>
#include <string>
#include <variant>

#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/concepts/alternative_c.hpp"

// Main ideas taken from https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjhxti2/

namespace utils {

    struct NotImplemented {
        std::string msg;
    };

    namespace _impl {

        template <typename... Ts> struct variant_traits {
            using types = std::tuple<Ts...>;
        };

        template <typename... Ts> struct variant_traits<std::variant<Ts...>> {
            using types = std::tuple<Ts...>;
        };

        template <typename FromVariant, typename ToVariant, std::size_t... I>
        ToVariant convert_variant_impl(FromVariant&& in, std::index_sequence<I...>) {
            using ToTypes = typename variant_traits<std::decay_t<ToVariant>>::types;

            return std::visit(
                [](auto&& arg) -> ToVariant {
                    using T = std::decay_t<decltype(arg)>;

                    static_assert(((std::is_same_v<T, std::tuple_element_t<I, ToTypes>> || ...)),
                                  "convert_variant: target variant must contain all source types");

                    return ToVariant{std::forward<T>(arg)};
                },
                std::forward<FromVariant>(in));
        }

        template <typename FromVariant, typename ToVariant> ToVariant convert_variant(FromVariant&& in) {
            using ToTypes = typename variant_traits<std::decay_t<ToVariant>>::types;
            constexpr std::size_t N = std::tuple_size_v<ToTypes>;
            return convert_variant_impl<FromVariant, ToVariant>(std::forward<FromVariant>(in),
                                                                std::make_index_sequence<N>{});
        }

    } // namespace _impl

    template <typename T, typename... Errs>
    using ExpectedType = std::expected<T, std::variant<NotImplemented, Errs...>>;

    // template <typename T, typename... Errs>
    // struct ExpectedType : std::expected<T, std::variant<NotImplemented, Errs...>> {
    //     using std::expected<T, std::variant<NotImplemented, Errs...>>::expected;

    //     /// A constructor that can take any of the unexpected types
    //     template <typename Err>
    //         requires utils::concepts::alternative_in_pack<Err, NotImplemented, Errs...>
    //     explicit ExpectedType(Err&& e)
    //         : std::expected<T, std::variant<NotImplemented, Errs...>>{std::unexpected(std::move(e))} {}

    //     // A constructor from a subset of errors
    //     template <class... FromArgs>
    //     explicit ExpectedType(std::variant<NotImplemented, FromArgs...>&& e)
    //         : std::expected<T, std::variant<NotImplemented, Errs...>>(
    //               std::unexpect, _impl::convert_variant<std::variant<NotImplemented, FromArgs...>,
    //                                                    std::variant<NotImplemented, Errs...>>(std::move(e))) {}

    //     /// Cast-move operator to convert to an `ExpectedType` with a superset of error types (note that the result
    //     type
    //     /// has to be the same)
    //     template <class... ToArgs> operator ExpectedType<T, ToArgs...>() && {
    //         if (this->has_value()) {
    //             return {std::move(*this)};
    //         } else {
    //             // std::variant<NotImplemented, Errs...> err = std::move(this->error());
    //             return std::visit(
    //                 [](auto&& arg) {
    //                     // using TErr = std::decay_t<decltype(arg)>;
    //                     return ExpectedType<T, ToArgs...>(std::move(arg));
    //                 },
    //                 std::move(this->error()));
    //         }
    //     }
    // };

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

// Required for std::format
template <typename... Args> struct std::formatter<std::variant<Args...>> : std::formatter<std::string> {
    auto format(const std::variant<Args...>& p, std::format_context& ctx) const {
        return std::visit(
            [&ctx, this](auto&& arg) {
                // using T = std::decay_t<decltype(arg)>;
                return this->formatter<std::string>::format(std::format("{}", arg), ctx);
            },
            p);
    }
};

template <> struct std::formatter<utils::NotImplemented> : std::formatter<std::string> {
    auto format(const utils::NotImplemented& p, std::format_context& ctx) const {
        return std::formatter<std::string>::format(std::format("NotImplemented: {}", p.msg), ctx);
    }
};

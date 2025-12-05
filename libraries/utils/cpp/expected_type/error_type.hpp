#pragma once

#include <variant>

#include "libraries/utils/cpp/concepts/alternative_c.hpp"

#include "./errors.hpp"

namespace utils {

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

    template <typename... Errs> struct ErrorType : std::variant<NotImplemented, Errs...> {
        using std::variant<NotImplemented, Errs...>::variant;

        // A constructor from a subset of types
        template <class... FromArgs>
        explicit ErrorType(ErrorType<FromArgs...>&& other)
            : std::variant<NotImplemented, Errs...>(
                  _impl::convert_variant<std::variant<NotImplemented, FromArgs...>,
                                         std::variant<NotImplemented, Errs...>>(std::move(other))) {}

        // Implicit cast operator to a superset of types
        template <class... ToArgs> operator ErrorType<ToArgs...>() && {
            return _impl::convert_variant<std::variant<NotImplemented, Errs...>,
                                          std::variant<NotImplemented, ToArgs...>>(std::move(*this));
        }

        // Check if the contained error matches the one in the input argument
        template <typename Err>
            requires utils::concepts::alternative_in_pack<Err, NotImplemented, Errs...>
        bool operator==(const Err& e) const {
            if (const Err* c = std::get_if<Err>(this)) {
                return *c == e;
            } else {
                return false;
            }
        }

        // Throw an exception using the underlying type
        void throw_exception() noexcept(false) {
            return std::visit([](auto&& arg) { throw arg; }, std::move(*this));
        }
    };

} // namespace utils

// Required for spdlog
template <typename... Args> struct fmt::formatter<utils::ErrorType<Args...>> : fmt::formatter<std::string> {
    auto format(const utils::ErrorType<Args...>& p, format_context& ctx) const -> decltype(ctx.out()) {
        return std::visit(
            [&ctx](const auto& arg) {
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
            [&ctx, this](const auto& arg) {
                // using T = std::decay_t<decltype(arg)>;
                return this->formatter<std::string>::format(std::format("{}", arg), ctx);
            },
            p);
    }
};

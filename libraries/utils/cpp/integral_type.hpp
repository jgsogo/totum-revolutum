#pragma once

#include <format>
#include <spdlog/spdlog.h>

namespace utils {
    template <typename T, typename TInteger> class IntegralType {
        static_assert(std::is_integral_v<TInteger>, "TInteger must be an integral type");

      public:
        constexpr explicit IntegralType(TInteger&& value) : value{std::move(value)} {}
        constexpr explicit IntegralType(TInteger value) : value{value} {}

        template <typename TInteger2> constexpr IntegralType(TInteger2 value) : value{value} {
            static_assert(std::is_integral_v<TInteger2>, "Tinteger2 must be an integral type");
        }

        operator TInteger() const { return value; }

        auto operator<=>(const IntegralType<T, TInteger>&) const = default;

      private:
        TInteger value;
    };
} // namespace utils

// Required for std::format
template <typename T, typename TInteger>
struct std::formatter<utils::IntegralType<T, TInteger>> : std::formatter<TInteger> {
    auto format(const utils::IntegralType<T, TInteger>& p, std::format_context& ctx) const {
        return std::formatter<TInteger>::format(p.value, ctx);
    }
};

// Required for spdlog
template <typename T, typename TInteger>
struct fmt::formatter<utils::IntegralType<T, TInteger>> : fmt::formatter<TInteger> {
    auto format(utils::IntegralType<T, TInteger> p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "{}", static_cast<TInteger>(p));
    }
};

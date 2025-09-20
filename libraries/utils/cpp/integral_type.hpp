#pragma once

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

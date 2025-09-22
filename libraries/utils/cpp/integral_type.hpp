#pragma once

#include <format>
#include <pqxx/pqxx>
#include <spdlog/spdlog.h>

namespace utils {
    template <typename T, typename TInteger> class IntegralType {
        static_assert(std::is_integral_v<TInteger>, "TInteger must be an integral type");

      public:
        IntegralType() = default;
        IntegralType(const IntegralType&) = default;
        IntegralType(IntegralType&&) = default;
        IntegralType& operator=(const IntegralType&) = default;
        IntegralType& operator=(IntegralType&&) = default;
        ~IntegralType() = default;

        constexpr explicit IntegralType(TInteger&& value) : value{std::move(value)} {}
        constexpr explicit IntegralType(TInteger value) : value{value} {}

        // template <typename TInteger2> constexpr IntegralType(TInteger2 value) : value{value} {
        //     static_assert(std::is_integral_v<TInteger2>, "Tinteger2 must be an integral type");
        // }

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

// // Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// // most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {

    template <typename T, typename TInteger>
    inline std::string const type_name<utils::IntegralType<T, TInteger>>{"utils::IntegralType<T, TInteger>"};

    template <typename T, typename TInteger> struct nullness<utils::IntegralType<T, TInteger>> : nullness<TInteger> {};

    template <typename T, typename TInteger> struct string_traits<utils::IntegralType<T, TInteger>> {
        static utils::IntegralType<T, TInteger> from_string(std::string_view text) {
            TInteger inner_value = string_traits<TInteger>::from_string(text);
            return utils::IntegralType<T, TInteger>{inner_value};
        }

        static zview to_buf(char* begin, char* end, const utils::IntegralType<T, TInteger>& value) {
            return string_traits<TInteger>::to_buf(begin, end, static_cast<TInteger>(value));
        }

        static char* into_buf(char* begin, char* end, const utils::IntegralType<T, TInteger>& value) {
            return string_traits<TInteger>::into_buf(begin, end, static_cast<TInteger>(value));
        }

        static std::size_t size_buffer(const utils::IntegralType<T, TInteger>& value) noexcept {
            return string_traits<TInteger>::size_buffer(static_cast<TInteger>(value));
        }
    };

} // namespace pqxx

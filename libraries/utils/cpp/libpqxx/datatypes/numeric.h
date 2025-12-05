

#pragma once

#include <string>

#include <decimal/decimal.h>
#include <pqxx/pqxx>
#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/expected_type/expected_type.hpp"

namespace utils::libpqxx {
    static const dec::decimal_format ENGLISH_DECIMAL_FORMAT{'.'};
    static const dec::decimal_format SPANISH_DECIMAL_FORMAT{','};

    using ParseNumericError = utils::errors::BaseError<"ParseNumericError">;

    template <std::size_t MaxDigits, std::size_t DecimalPlaces> struct Numeric {
        // dec::decimal maximum number of digits is 18 (uses 64 bit integer under the hood)
        static_assert(MaxDigits <= 18, "MaxDigits cannot excceed 18");
        using InnerType = dec::decimal<DecimalPlaces>;

        static const std::size_t max_digits = MaxDigits;

        Numeric() = default;
        explicit Numeric(InnerType&& inner) : value{std::move(inner)} {}
        explicit Numeric(const InnerType& inner) : value{inner} {}
        Numeric(Numeric&&) noexcept = default;
        Numeric(const Numeric&) = default;
        Numeric& operator=(Numeric&&) noexcept = default;

        Numeric& operator+=(const Numeric& other);
        friend Numeric operator+(const Numeric& lhs, const Numeric& rhs);
        auto operator<=>(const Numeric<MaxDigits, DecimalPlaces>&) const = default;

        operator std::string() const { return dec::toString(value, ENGLISH_DECIMAL_FORMAT); }

        static ExpectedType<Numeric<MaxDigits, DecimalPlaces>, ParseNumericError>
        parse(const std::string& input, const dec::decimal_format& fmt) {
            InnerType output;

            std::istringstream is(input);
            bool success = dec::fromStream(is, fmt, output);
            if (!success) {
                return tl::unexpected{ParseNumericError{std::string{input}}};
            }
            return {Numeric<MaxDigits, DecimalPlaces>{std::move(output)}};
        };

        InnerType value;
    };

    template <std::size_t MaxDigits, std::size_t DecimalPlaces>
    inline Numeric<MaxDigits, DecimalPlaces>&
    Numeric<MaxDigits, DecimalPlaces>::operator+=(const Numeric<MaxDigits, DecimalPlaces>& other) {
        value += other.value;
        return *this;
    }

    template <std::size_t MaxDigits, std::size_t DecimalPlaces>
    inline Numeric<MaxDigits, DecimalPlaces> operator*(const Numeric<MaxDigits, DecimalPlaces>& lhs,
                                                       const Numeric<MaxDigits, DecimalPlaces>& rhs) {
        // TODO: Add testing. Are we loosing precision here? Is overflow possible?
        return Numeric<MaxDigits, DecimalPlaces>{lhs.value * rhs.value};
    }

    template <std::size_t MaxDigits, std::size_t DecimalPlaces>
    inline Numeric<MaxDigits, DecimalPlaces> operator/(const Numeric<MaxDigits, DecimalPlaces>& lhs,
                                                       const Numeric<MaxDigits, DecimalPlaces>& rhs) {
        // TODO: Add testing. Are we loosing precision here? Is overflow possible?
        return Numeric<MaxDigits, DecimalPlaces>{lhs.value / rhs.value};
    }

    template <std::size_t MaxDigits, std::size_t DecimalPlaces>
    Numeric<MaxDigits, DecimalPlaces> operator+(const Numeric<MaxDigits, DecimalPlaces>& lhs,
                                                const Numeric<MaxDigits, DecimalPlaces>& rhs) {
        return Numeric<MaxDigits, DecimalPlaces>{lhs.value + rhs.value};
    }

} // namespace utils::libpqxx

// Required for std::format
template <std::size_t MaxDigits, std::size_t DecimalPlaces>
struct std::formatter<utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>> : std::formatter<std::string> {
    auto format(const utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>& p, std::format_context& ctx) const {
        return std::formatter<std::string>::format(dec::toString(p.value, utils::libpqxx::ENGLISH_DECIMAL_FORMAT), ctx);
    }
};

// Required for spdlog
template <std::size_t MaxDigits, std::size_t DecimalPlaces>
struct fmt::formatter<utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>> : fmt::formatter<std::string> {
    auto format(utils::libpqxx::Numeric<MaxDigits, DecimalPlaces> p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "{}", dec::toString(p.value, utils::libpqxx::ENGLISH_DECIMAL_FORMAT));
    }
};

// Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {

    static const dec::decimal_format POSTGRES_DECIMAL_FORMAT{'.'}; // Does this depend on some PostgreSQL locale?

    template <std::size_t MaxDigits, std::size_t DecimalPlaces>
    inline std::string const type_name<utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>>{
        "utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>"};

    template <std::size_t MaxDigits, std::size_t DecimalPlaces>
    struct nullness<utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>>
        : no_null<utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>> {};

    template <std::size_t MaxDigits, std::size_t DecimalPlaces>
    struct string_traits<utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>> {
        static utils::libpqxx::Numeric<MaxDigits, DecimalPlaces> from_string(std::string_view text) {
            dec::decimal<DecimalPlaces> inner_value =
                dec::fromString<dec::decimal<DecimalPlaces>>(std::string{text}, POSTGRES_DECIMAL_FORMAT);
            return utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>{inner_value};
        }

        static zview to_buf(char* begin, char* end, const utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>& value) {
            // auto string = date::format(utils::libpqxx::DATE_FORMAT, value);
            std::string string = dec::toString(value.value, POSTGRES_DECIMAL_FORMAT);

            if (std::distance(begin, end) < static_cast<signed long>(string.size() + 1)) {
                throw pqxx::conversion_overrun{"could not convert utils::libpqxx::Date"};
            }
            std::copy(string.cbegin(), string.cend(), begin);
            begin[string.size()] = '\0';
            return zview{begin, string.size()};
        }

        static char* into_buf(char* begin, char* end, const utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>& value) {
            auto v = to_buf(begin, end, value);
            return begin + v.size() + 2; // past the '\0'
        }

        static std::size_t size_buffer(const utils::libpqxx::Numeric<MaxDigits, DecimalPlaces>& value) noexcept {
            return MaxDigits + 1 + 1;
        }
    };
} // namespace pqxx

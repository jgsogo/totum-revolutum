

#pragma once

#include <string>

#include <date/date.h>
#include <pqxx/pqxx>
#include <spdlog/spdlog.h>

namespace utils::libpqxx {
    using Date = date::year_month_day; // FIXME: Probably std::chrono::year_month_day is enough, and we don't need the
                                       // extra @date library

    constexpr static std::string DATE_FORMAT = "%Y-%m-%d";
} // namespace utils::libpqxx

// Required for std::format
template <> struct std::formatter<utils::libpqxx::Date> : std::formatter<std::string> {
    auto format(const utils::libpqxx::Date& p, std::format_context& ctx) const {
        return std::formatter<std::string>::format(date::format(utils::libpqxx::DATE_FORMAT, p), ctx);
    }
};

// Required for spdlog
template <> struct fmt::formatter<utils::libpqxx::Date> : fmt::formatter<std::string> {
    auto format(utils::libpqxx::Date p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "{}", date::format(utils::libpqxx::DATE_FORMAT, p));
    }
};

// // Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// // most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {

    template <> inline std::string const type_name<utils::libpqxx::Date>{"utils::libpqxx::Date"};

    template <> struct nullness<utils::libpqxx::Date> : no_null<utils::libpqxx::Date> {};

    template <> struct string_traits<utils::libpqxx::Date> {
        static utils::libpqxx::Date from_string(std::string_view text) {
            std::istringstream in{std::string{text}};
            utils::libpqxx::Date date{};

            in >> date::parse(utils::libpqxx::DATE_FORMAT, date);
            if (!in) {
                throw pqxx::conversion_error(std::format("Error parsing date from {}", text));
            }
            return date;
        }

        static zview to_buf(char* begin, char* end, const utils::libpqxx::Date& value) {
            auto string = date::format(utils::libpqxx::DATE_FORMAT, value);

            if (std::distance(begin, end) < static_cast<signed long>(string.size() + 1)) {
                throw pqxx::conversion_overrun{"could not convert utils::libpqxx::Date"};
            }
            std::copy(string.cbegin(), string.cend(), begin);
            begin[string.size()] = '\0';
            return zview{begin, string.size()};
        }

        static char* into_buf(char* begin, char* end, const utils::libpqxx::Date& value) {
            auto v = to_buf(begin, end, value);
            return begin + v.size() + 2; // past the '\0'
        }

        static std::size_t size_buffer(const utils::libpqxx::Date& value) noexcept { return 10 + 1; }
    };
} // namespace pqxx

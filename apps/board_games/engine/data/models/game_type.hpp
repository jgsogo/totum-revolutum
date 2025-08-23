#pragma once

#include <string>

#include <pqxx/pqxx>
#include <spdlog/spdlog.h>

namespace data {

    namespace _detail {
        template <typename T> class StringType {
          public:
            constexpr explicit StringType(std::string&& value) : value{std::move(value)} {}

            operator std::string_view() const { return value; }

            auto operator<=>(const StringType<T>&) const = default;

          private:
            std::string value;
        };
    } // namespace _detail

    using GameType = _detail::StringType<class GameTypeTag>;
} // namespace data

template <typename T> struct fmt::formatter<data::_detail::StringType<T>> : fmt::formatter<std::string> {
    auto format(data::_detail::StringType<T> participant, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "{}", static_cast<std::string_view>(participant));
    }
};

template <typename T> struct std::formatter<data::_detail::StringType<T>> : std::formatter<std::string_view> {
    auto format(const data::_detail::StringType<T>& obj, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(static_cast<std::string_view>(obj), ctx);
    }
};

// // Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// // most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {
    template <> inline std::string const type_name<data::GameType>{"GameType"};

    template <> struct nullness<data::GameType> : no_null<data::GameType> {};

    template <> struct string_traits<data::GameType> {
        static data::GameType from_string(std::string_view text) { return data::GameType{std::string{text}}; }

        static zview to_buf(char* begin, char* end, const data::GameType& value) {
            auto string = std::string{value};

            if (std::distance(begin, end) < static_cast<signed long>(string.size() + 1)) {
                throw pqxx::conversion_overrun{"could not convert GameType"};
            }
            std::copy(string.cbegin(), string.cend(), begin);
            begin[string.size()] = '\0';
            return zview{begin, string.size()};
        }

        static char* into_buf(char* begin, char* end, const data::GameType& value) {
            auto v = to_buf(begin, end, value);
            return begin + v.size() + 2; // past the '\0'
        }

        static std::size_t size_buffer(const data::GameType& value) noexcept {
            return std::string(value).size() + 1; // include trailing '\0'
        }
    };
} // namespace pqxx

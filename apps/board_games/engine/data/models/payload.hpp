#pragma once

#include <string>

namespace data {

    namespace _detail {
        template <typename T> class Payload {
          public:
            Payload() = delete;
            Payload(const Payload&) = delete;
            explicit Payload(std::string&& payload) : payload{payload} {};
            explicit Payload(Payload&& payload) = default;
            explicit Payload(const Payload&& payload) = default;

            operator std::string_view() const { return payload; }

          protected:
            std::string payload;
        };
    } // namespace _detail

    using GameStatePayload = _detail::Payload<class GameStatePayloadTypeTag>;
    using GameActionPayload = _detail::Payload<class GameActionPayloadTypeTag>;
    using EventLogPayload = _detail::Payload<class EventLogPayloadTypeTag>;

} // namespace data

// // Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// // most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {
    template <> inline std::string const type_name<data::GameStatePayload>{"GameStatePayload"};
    template <> inline std::string const type_name<data::GameActionPayload>{"GameActionPayload"};
    template <> inline std::string const type_name<data::EventLogPayload>{"EventLogPayload"};

    template <typename T> struct nullness<data::_detail::Payload<T>> : no_null<data::_detail::Payload<T>> {};

    template <typename T> struct string_traits<data::_detail::Payload<T>> {
        static data::_detail::Payload<T> from_string(std::string_view text) {
            return data::_detail::Payload<T>{std::string{text}};
        }

        static zview to_buf(char* begin, char* end, const data::_detail::Payload<T>& value) {
            auto string = std::string{value};

            if (std::distance(begin, end) < static_cast<signed long>(string.size() + 1)) {
                throw pqxx::conversion_overrun{"could not convert data::_detail::Payload<T>"};
            }
            std::copy(string.cbegin(), string.cend(), begin);
            begin[string.size()] = '\0';
            return zview{begin, string.size()};
        }

        static char* into_buf(char* begin, char* end, const data::_detail::Payload<T>& value) {
            auto v = to_buf(begin, end, value);
            return begin + v.size() + 2; // past the '\0'
        }

        static std::size_t size_buffer(const data::_detail::Payload<T>& value) noexcept {
            return std::string(value).size() + 1; // include trailing '\0'
        }
    };
} // namespace pqxx

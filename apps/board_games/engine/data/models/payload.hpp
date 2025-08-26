#pragma once

#include <string>

namespace data {

    namespace _detail {
        template <typename T> class Payload {
          public:
            Payload() = delete;
            Payload(const Payload&) = delete;
            explicit Payload(std::vector<std::byte>&& payload) : _payload{std::move(payload)} {};
            explicit Payload(Payload&& payload) = default;

            operator std::span<const std::byte>() const { return {_payload.cbegin(), _payload.size()}; }
            const void* data() const { return _payload.data(); }
            std::size_t size() const { return _payload.size(); }
            operator pqxx::bytes_view() const { return pqxx::bytes_view{_payload.begin(), _payload.end()}; }
            pqxx::bytes_view payload() const { return pqxx::bytes_view{_payload.begin(), _payload.end()}; }

          protected:
            std::vector<std::byte> _payload;
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
            SPDLOG_ERROR("from_string. We retrieve {}: '{}'", text.size(), text);
            std::vector<std::byte> data(text.size());
            std::memcpy(data.data(), text.data(), text.size());
            return data::_detail::Payload<T>{std::move(data)};
        }

        static zview to_buf(char* begin, char* end, const data::_detail::Payload<T>& value) {
            auto data = static_cast<std::span<const std::byte>>(value);
            if (std::distance(begin, end) < static_cast<signed long>(data.size() + 1)) {
                throw pqxx::conversion_overrun{"could not convert data::_detail::Payload<T>"};
            }
            std::memcpy(begin, data.data(), data.size());
            begin[data.size()] = '\0';
            SPDLOG_ERROR("to_buf. We have {} available. We serialize {}: '{}'", std::distance(begin, end), data.size(),
                         std::string_view{begin, end});
            return zview{begin, data.size()};
        }

        static char* into_buf(char* begin, char* end, const data::_detail::Payload<T>& value) {
            auto v = to_buf(begin, end, value);
            return begin + v.size() + 2; // past the '\0'
        }

        static std::size_t size_buffer(const data::_detail::Payload<T>& value) noexcept {
            return value.size() + 1; // include trailing '\0'
        }
    };
} // namespace pqxx

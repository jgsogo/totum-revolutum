#pragma once

#include <pqxx/pqxx>
#include <spdlog/spdlog.h>

#include "uuid.hpp"

namespace data {
    using RoomUUID = _detail::UUID<_detail::UUIDType::Room>;
} // namespace data

template <> struct fmt::formatter<data::RoomUUID> : fmt::formatter<std::string> {
    auto format(data::RoomUUID room, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "room-{}", static_cast<std::string_view>(room));
    }
};

// Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {
    template <> inline std::string const type_name<data::RoomUUID>{"RoomUUID"};

    template <> struct nullness<data::RoomUUID> {
        static constexpr bool has_null{true};
        static constexpr bool always_null{false};

        static bool is_null(const data::RoomUUID& value) { return value.is_null(); }

        [[nodiscard]] static data::RoomUUID null() { return data::RoomUUID::null(); }
    };

    template <> struct string_traits<data::RoomUUID> {
        static data::RoomUUID from_string(std::string_view text) { return data::RoomUUID{std::string{text}}; }

        static zview to_buf(char* begin, char* end, const data::RoomUUID& value) {
            auto string = std::string{value};

            if (std::distance(begin, end) < static_cast<signed long>(string.size() + 1)) {
                throw pqxx::conversion_overrun{"could not convert RoomUUID"};
            }
            std::copy(string.cbegin(), string.cend(), begin);
            begin[string.size()] = '\0';
            return zview{begin, string.size()};
        }

        static char* into_buf(char* begin, char* end, const data::RoomUUID& value) {
            auto v = to_buf(begin, end, value);
            return begin + v.size() + 2; // past the '\0'
        }

        static std::size_t size_buffer(const data::RoomUUID&) noexcept {
            return 36 + 1; // include trailing '\0'
        }
    };
} // namespace pqxx

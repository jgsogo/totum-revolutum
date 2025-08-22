#pragma once

#include <pqxx/pqxx>

#include "uuid.hpp"

namespace data {
    using ParticipantUUID = _detail::UUID<_detail::UUIDType::Participant>;
} // namespace data

// Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {
    template <> inline std::string const type_name<data::ParticipantUUID>{"ParticipantUUID"};

    template <> struct nullness<data::ParticipantUUID> {
        static constexpr bool has_null{true};
        static constexpr bool always_null{false};

        static bool is_null(const data::ParticipantUUID& value) { return value.is_null(); }

        [[nodiscard]] static data::ParticipantUUID null() { return data::ParticipantUUID::null(); }
    };

    template <> struct string_traits<data::ParticipantUUID> {
        static data::ParticipantUUID from_string(std::string_view text) {
            return data::ParticipantUUID{std::string{text}};
        }

        static zview to_buf(char* begin, char* end, const data::ParticipantUUID& value) {
            auto string = std::string{value};

            if (std::distance(begin, end) < static_cast<signed long>(string.size() + 1)) {
                throw pqxx::conversion_overrun{"could not convert ParticipantUUID"};
            }
            std::copy(string.cbegin(), string.cend(), begin);
            begin[string.size()] = '\0';
            return zview{begin, string.size()};
        }

        static char* into_buf(char* begin, char* end, const data::ParticipantUUID& value) {
            auto v = to_buf(begin, end, value);
            return begin + v.size() + 2; // past the '\0'
        }

        static std::size_t size_buffer(const data::ParticipantUUID&) noexcept {
            return 36 + 1; // include trailing '\0'
        }
    };
} // namespace pqxx

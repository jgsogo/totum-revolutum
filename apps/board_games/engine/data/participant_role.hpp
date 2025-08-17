#pragma once

#include <spdlog/spdlog.h>
// #include <spdlog/fmt/ostr.h>

namespace data {
    enum class ParticipantRole {
        PLAYER,
        SPECTATOR,
    };
}

template <> struct fmt::formatter<data::ParticipantRole> : fmt::formatter<std::string> {
    auto format(data::ParticipantRole role, format_context& ctx) const -> decltype(ctx.out()) {
        switch (role) {
        case data::ParticipantRole::PLAYER:
            return fmt::format_to(ctx.out(), "player");
        case data::ParticipantRole::SPECTATOR:
            return fmt::format_to(ctx.out(), "spectator");
        }
    }
};

// // Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// // most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {
    template <> inline std::string const type_name<data::ParticipantRole>{"ParticipantRole"};

    template <> struct nullness<data::ParticipantRole> : no_null<data::ParticipantRole> {};

    template <> struct string_traits<data::ParticipantRole> {
        static data::ParticipantRole from_string(std::string_view text) {
            if (text == "player")
                return data::ParticipantRole::PLAYER;
            else if (text == "spectator")
                return data::ParticipantRole::SPECTATOR;
            else
                throw pqxx::conversion_error(std::string{text});
        }

        static zview to_buf(char* begin, char* end, const data::ParticipantRole& value) {
            // std::string string = std::format("{}", value); // FIXME: This should work, but
            // https://github.com/llvm/llvm-project/issues/66466
            std::string string;
            switch (value) {
            case data::ParticipantRole::PLAYER:
                string = "player";
                break;
            case data::ParticipantRole::SPECTATOR:
                string = "spectator";
                break;
            }

            if (std::distance(begin, end) < static_cast<signed long>(string.size() + 1)) {
                throw pqxx::conversion_overrun{"could not convert ParticipantRole"};
            }
            std::copy(string.cbegin(), string.cend(), begin);
            begin[string.size()] = '\0';
            return zview{begin, string.size()};
        }

        static char* into_buf(char* begin, char* end, const data::ParticipantRole& value) {
            auto v = to_buf(begin, end, value);
            return begin + v.size() + 2; // past the '\0'
        }

        static std::size_t size_buffer(const data::ParticipantRole& value) noexcept {
            return 9 + 1; // include trailing '\0'
        }
    };
} // namespace pqxx

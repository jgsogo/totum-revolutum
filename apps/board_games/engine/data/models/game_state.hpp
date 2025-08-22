#pragma once

#include <spdlog/spdlog.h>
// #include <spdlog/fmt/ostr.h>

namespace data {

    static constexpr std::string_view GameStatePlaying = "playing";
    static constexpr std::string_view GameStateWaiting = "waiting";
    static constexpr std::string_view GameStateFinished = "finished";

    enum class GameState {
        PLAYING,
        WAITING,
        FINISHED,
    };

    inline tl::expected<GameState, std::string> game_state_from_string(std::string_view state) {
        if (state == GameStatePlaying)
            return {data::GameState::PLAYING};
        else if (state == GameStateWaiting)
            return {data::GameState::WAITING};
        else if (state == GameStateFinished)
            return {data::GameState::FINISHED};
        else
            return tl::unexpected{std::string{state}};
    }

    inline std::string_view game_state_to_string(const GameState& state) {
        switch (state) {
        case data::GameState::PLAYING:
            return GameStatePlaying;
        case data::GameState::WAITING:
            return GameStateWaiting;
        case data::GameState::FINISHED:
            return GameStateFinished;
        }
    }
} // namespace data

template <> struct fmt::formatter<data::GameState> : fmt::formatter<std::string> {
    auto format(data::GameState value, format_context& ctx) const -> decltype(ctx.out()) {
        switch (value) {
        case data::GameState::PLAYING:
            return fmt::format_to(ctx.out(), data::GameStatePlaying);

        case data::GameState::WAITING:
            return fmt::format_to(ctx.out(), data::GameStateWaiting);

        case data::GameState::FINISHED:
            return fmt::format_to(ctx.out(), data::GameStateFinished);
        }

        return fmt::format_to(ctx.out(), game_state_to_string(state));
    }
};

// // Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// // most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {
    template <> inline std::string const type_name<data::GameState>{"GameState"};

    template <> struct nullness<data::GameState> : no_null<data::GameState> {};

    template <> struct string_traits<data::GameState> {
        static data::GameState from_string(std::string_view text) {
            auto r = data::game_state_from_string(text);
            if (r.has_value()) {
                return r.value();
            } else {
                throw pqxx::conversion_error(std::string{text});
            }
        }

        static zview to_buf(char* begin, char* end, const data::GameState& value) {
            // std::string string = std::format("{}", value); // FIXME: This should work, but
            // https://github.com/llvm/llvm-project/issues/66466
            std::string_view string = data::game_state_to_string(value);

            if (std::distance(begin, end) < static_cast<signed long>(string.size() + 1)) {
                throw pqxx::conversion_overrun{"could not convert GameState"};
            }
            std::copy(string.cbegin(), string.cend(), begin);
            begin[string.size()] = '\0';
            return zview{begin, string.size()};
        }

        static char* into_buf(char* begin, char* end, const data::GameState& value) {
            auto v = to_buf(begin, end, value);
            return begin + v.size() + 2; // past the '\0'
        }

        static std::size_t size_buffer(const data::GameState& value) noexcept {
            return 9 + 1; // include trailing '\0'
        }
    };
} // namespace pqxx

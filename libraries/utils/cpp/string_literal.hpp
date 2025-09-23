#pragma once

#include <spdlog/spdlog.h>
#include <string>

namespace utils {
    /**
     * Literal class type that wraps a constant expression string.
     *
     * Uses implicit conversion to allow templates to *seemingly* accept constant strings.
     *
     * Taken from https://ctrpeach.io/posts/cpp20-string-literal-template-parameters/
     */
    template <size_t N> struct StringLiteral {
        constexpr StringLiteral(const char (&str)[N]) { std::copy_n(str, N, value); }

        constexpr operator std::string_view() const { return std::string_view{value}; }
        bool operator==(const std::string_view& other) const { return std::string_view{value} == other; };
        char value[N];
    };

    template <size_t N> std::ostream& operator<<(std::ostream& os, const StringLiteral<N>& p) {
        os << p.value;
        return os;
    }
} // namespace utils

// Required for std::format
template <size_t N> struct std::formatter<utils::StringLiteral<N>> : std::formatter<std::string> {
    auto format(utils::StringLiteral<N> p, format_context& ctx) const {
        return formatter<string>::format(std::format("{}", p.value), ctx);
    }
};

// Required for spdlog
template <size_t N> struct fmt::formatter<utils::StringLiteral<N>> : fmt::formatter<std::string> {
    auto format(utils::StringLiteral<N> p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "{}", p.value);
    }
};

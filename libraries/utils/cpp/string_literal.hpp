#pragma once

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

        char value[N];
    };
} // namespace utils

// Required for std::format
template <size_t N> struct std::formatter<utils::StringLiteral<N>> : std::formatter<std::string> {
    auto format(utils::StringLiteral<N> p, format_context& ctx) const {
        return formatter<string>::format(std::format("{}", p.value), ctx);
    }
};

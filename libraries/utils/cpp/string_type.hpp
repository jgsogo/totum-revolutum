#pragma once

#include <spdlog/spdlog.h>
#include <string>

namespace utils {
    template <typename T> class StringType {
      public:
        constexpr explicit StringType(std::string&& value) : value{std::move(value)} {}
        constexpr explicit StringType(std::string_view value) : value{std::string(value)} {}

        constexpr operator std::string_view() const { return value; }

        auto operator<=>(const StringType<T>&) const = default;
        // FIXME: Why these operators are not created by the spaceship one above?
        bool operator==(const StringType<T>& other) const = default;

        bool operator!=(const StringType<T>& other) const { return value != other.value; };
        bool operator==(const std::string& other) const { return value == other; };

        template <typename TT> friend std::ostream& operator<<(std::ostream&, const StringType<TT>&);

      private:
        std::string value;
    };

    template <typename T> std::ostream& operator<<(std::ostream& os, const StringType<T>& p) {
        os << p.value;
        return os;
    }
} // namespace utils

// Required for std::format
template <typename T> struct std::formatter<utils::StringType<T>> : std::formatter<std::string> {
    auto format(utils::StringType<T> p, format_context& ctx) const {
        return formatter<string>::format(std::format("{}", static_cast<std::string_view>(p)), ctx);
    }
};

// Required for spdlog
template <typename T> struct fmt::formatter<utils::StringType<T>> : fmt::formatter<std::string> {
    auto format(utils::StringType<T> p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "{}", static_cast<std::string_view>(p));
    }
};

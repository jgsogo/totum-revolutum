#pragma once

namespace utils {
    template <typename T> class StringType {
      public:
        constexpr explicit StringType(std::string&& value) : value{std::move(value)} {}
        constexpr explicit StringType(std::string_view value) : value{std::string(value)} {}

        constexpr operator std::string_view() const { return value; }

        auto operator<=>(const StringType<T>&) const = default;

      private:
        std::string value;
    };
} // namespace utils

// Required for std::format
template <typename T> struct std::formatter<utils::StringType<T>> : std::formatter<std::string> {
    auto format(utils::StringType<T> p, format_context& ctx) const {
        return formatter<string>::format(std::format("{}", static_cast<std::string_view>(p)), ctx);
    }
};

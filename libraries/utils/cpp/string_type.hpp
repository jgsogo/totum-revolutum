#pragma once

namespace utils {
    template <typename T> class StringType {
      public:
        constexpr explicit StringType(std::string&& value) : value{std::move(value)} {}
        constexpr explicit StringType(std::string_view value) : value{std::string(value)} {}

        operator std::string_view() const { return value; }

        auto operator<=>(const StringType<T>&) const = default;

      private:
        std::string value;
    };
} // namespace utils

#pragma once

#include "libraries/utils/cpp/integral_type.hpp"
#include <cstdint>
#include <variant>

namespace utils::db {
    using IdType = utils::IntegralType<class DatabaseId, uint64_t>;
    using Id = std::variant<std::monostate, IdType>;

    constexpr bool is_null(const utils::db::Id& value) { return value.index() == 0; }
} // namespace utils::db

// Required for std::format
template <> struct std::formatter<utils::db::Id> : std::formatter<std::string_view> {
    template <class FmtContext> FmtContext::iterator format(const utils::db::Id& p, FmtContext& ctx) const {
        return std::visit(
            [&ctx, this](auto&& arg) -> FmtContext::iterator {
                using T = std::decay_t<decltype(arg)>;
                if constexpr (std::is_same_v<T, std::monostate>) {
                    return std::formatter<std::string_view>::format("<not-set>", ctx);
                } else if constexpr (std::is_same_v<T, utils::db::IdType>) {
                    return std::formatter<std::string_view>::format(std::format("{}", arg), ctx);
                } else {
                    static_assert(false, "non-exhaustive visitor!");
                }
            },
            p);
    }
};

// Required for spdlog
template <> struct fmt::formatter<utils::db::Id> : fmt::formatter<std::string_view> {
    auto format(utils::db::Id p, fmt::format_context& ctx) const -> decltype(ctx.out()) {
        return std::visit(
            [&ctx, this](auto&& arg) -> decltype(ctx.out()) {
                using T = std::decay_t<decltype(arg)>;
                if constexpr (std::is_same_v<T, std::monostate>) {
                    return fmt::formatter<std::string_view>::format("<not-set>", ctx);
                } else if constexpr (std::is_same_v<T, utils::db::IdType>) {
                    return fmt::formatter<std::string_view>::format(std::format("{}", arg), ctx);
                } else {
                    static_assert(false, "non-exhaustive visitor!");
                }
            },
            p);
    }
};

namespace pqxx {

    template <> inline std::string const type_name<utils::db::Id>{"utils::db::Id"};

    template <> struct nullness<utils::db::Id> {
        // Does T have a value that should translate to an SQL null?
        static constexpr bool has_null{true};

        // Does this C++ type always denote an SQL null, like with nullptr_t?
        static constexpr bool always_null{false};

        static bool is_null(const utils::db::Id& value) { return utils::db::is_null(value); }

        [[nodiscard]] static utils::db::Id null() {
            // Return a null value.
            return {std::monostate{}};
        }
    };

    template <> struct string_traits<utils::db::Id> {
        static utils::db::Id from_string(std::string_view text) {
            uint64_t inner_value = string_traits<uint64_t>::from_string(text);
            return utils::db::Id{utils::db::IdType{inner_value}};
        }

        static zview to_buf(char* begin, char* end, const utils::db::Id& value) {
            return std::visit(
                [&begin, &end](auto&& arg) -> zview {
                    using T = std::decay_t<decltype(arg)>;
                    if constexpr (std::is_same_v<T, std::monostate>) {
                        throw pqxx::conversion_error{"Trying to save non-state ID to the database"};
                    } else if constexpr (std::is_same_v<T, utils::db::IdType>) {
                        return string_traits<utils::db::IdType>::to_buf(begin, end, arg);
                    } else {
                        static_assert(false, "non-exhaustive visitor!");
                    }
                },
                value);
        }

        static char* into_buf(char* begin, char* end, const utils::db::Id& value) {
            return std::visit(
                [&begin, &end](auto&& arg) -> char* {
                    using T = std::decay_t<decltype(arg)>;
                    if constexpr (std::is_same_v<T, std::monostate>) {
                        throw pqxx::conversion_error{"Trying to save non-state ID to the database"};
                    } else if constexpr (std::is_same_v<T, utils::db::IdType>) {
                        return string_traits<utils::db::IdType>::into_buf(begin, end, arg);
                    } else {
                        static_assert(false, "non-exhaustive visitor!");
                    }
                },
                value);

            // return string_traits<utils::db::IdType>::into_buf(begin, end, static_cast<utils::db::IdType>(value));
        }

        static std::size_t size_buffer(const utils::db::Id& value) noexcept {
            return string_traits<utils::db::IdType>::size_buffer(utils::db::IdType{});
        }
    };

} // namespace pqxx

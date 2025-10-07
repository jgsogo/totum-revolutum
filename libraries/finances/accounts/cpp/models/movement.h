#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"
#include <magic_enum/magic_enum.hpp>

#include "account.h"
#include "hierarchy_tree.h"
#include "transaction.h"
#include "types/amount.h"
#include "types/id.h"

namespace finances::accounts::models {

    class MovementManager;

    enum class MovementDirection : int32_t { IN = 0, OUT = 1 };

    struct Movement {
        using Manager = MovementManager;

        Id id;
        std::pair<decltype(Transaction::id), decltype(Transaction::name)> transaction;
        std::pair<decltype(MovementType::id), decltype(MovementType::name)> type;
        MovementDirection direction;
        decltype(AccountType::id) account_id;
        utils::libpqxx::Date date_value;
        // Fx fx;
        Amount amount;
    };

    class MovementManager : public ModelManager<Movement> {
      public:
        tl::expected<std::vector<Movement>, Error> all(Id account_id);
    };

} // namespace finances::accounts::models

// Required for spdlog
template <> struct fmt::formatter<finances::accounts::models::MovementDirection> : fmt::formatter<std::string> {
    auto format(finances::accounts::models::MovementDirection p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "{}", magic_enum::enum_name(p));
    }
};

// // Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// // most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {
    using namespace finances::accounts::models;

    template <> inline std::string const type_name<MovementDirection>{"MovementDirection"};

    template <> struct nullness<MovementDirection> : no_null<MovementDirection> {};

    template <> struct string_traits<MovementDirection> {
        static MovementDirection from_string(std::string_view text) {
            int32_t value = string_traits<int32_t>::from_string(text);
            auto direction = magic_enum::enum_cast<MovementDirection>(value);
            if (!direction) {
                throw pqxx::conversion_error(std::format("Error parsing movement-direction from {}", text));
            }
            return direction.value();
        }

        static zview to_buf(char* begin, char* end, const MovementDirection& value) {
            return string_traits<int32_t>::to_buf(begin, end, magic_enum::enum_integer(value));
        }

        static char* into_buf(char* begin, char* end, const MovementDirection& value) {
            return string_traits<int32_t>::into_buf(begin, end, magic_enum::enum_integer(value));
        }

        static std::size_t size_buffer(const MovementDirection& value) noexcept {
            return string_traits<int32_t>::size_buffer(magic_enum::enum_integer(value));
        }
    };

} // namespace pqxx

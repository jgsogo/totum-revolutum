#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"
#include <magic_enum/magic_enum.hpp>

#include "account.h"
#include "hierarchy_tree.h"
#include "transaction.h"
#include "types/amount.h"

namespace finances::accounts::models {

    enum class MovementDirection : int32_t { IN = 0, OUT = 1 };

    struct Movement {
        utils::db::Id id;
        std::pair<decltype(Transaction::id), decltype(Transaction::name)> transaction;
        std::pair<decltype(MovementType::id), decltype(MovementType::name)> type;
        MovementDirection direction;
        std::pair<decltype(Account::id), decltype(Account::name)> account;
        utils::libpqxx::Date date_value;
        // Fx fx;
        Amount amount;
    };

} // namespace finances::accounts::models

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

namespace utils::db {

    template <>
    template <>
    std::vector<finances::accounts::models::Movement>
    utils::db::ModelManager<finances::accounts::models::Movement>::_filter_by_fk<finances::accounts::models::Account>(
        pqxx::work&, const ModelData<finances::accounts::models::Account>::Id& id);

    template <>
    template <>
    std::vector<finances::accounts::models::Movement>
    utils::db::ModelManager<finances::accounts::models::Movement>::_filter_by_fk<
        finances::accounts::models::Transaction>(pqxx::work&,
                                                 const ModelData<finances::accounts::models::Transaction>::Id& id);

} // namespace utils::db

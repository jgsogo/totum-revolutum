#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"

#include "libraries/utils/cpp/libpqxx/orm/id.h"
#include "libraries/utils/cpp/libpqxx/orm/manager.h"

namespace finances::accounts::models {

    struct TransactionGroup {
        utils::db::Id id;

        std::string name;
        std::optional<std::string> description;

        utils::libpqxx::Date open;
        std::optional<utils::libpqxx::Date> close;
    };

    struct Transaction {
        utils::db::Id id;
        std::string name;
        std::optional<std::string> description;

        std::optional<std::pair<decltype(TransactionGroup::id), decltype(TransactionGroup::name)>> group;
    };
} // namespace finances::accounts::models

namespace utils::db {

    template <>
    ExpectedType<finances::accounts::models::Transaction, ErrorNotFound, ErrorMultipleFound>
    ModelManager<finances::accounts::models::Transaction>::_get(
        pqxx::work&, const decltype(finances::accounts::models::Transaction::id)&);

}

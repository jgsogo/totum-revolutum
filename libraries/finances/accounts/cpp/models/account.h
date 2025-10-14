#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"
// #include "libraries/utils/cpp/libpqxx/orm/manager.h"
#include "libraries/utils/cpp/libpqxx/orm/manager.h"

#include "custodian.h"
#include "hierarchy_tree.h"
#include "model_manager.hpp"
#include "types/ccy.h"
#include "types/id.h"

namespace finances::accounts::models {

    struct Account {
        Id id;
        std::string name;
        std::optional<std::string> description;
        std::optional<std::string> identifier;
        Ccy ccy;
        utils::libpqxx::Date open;
        std::optional<utils::libpqxx::Date> close;
        std::pair<decltype(AccountType::id), decltype(AccountType::name)> type;
        std::pair<decltype(Custodian::id), decltype(Custodian::name)> custodian;
        bool is_numerable;
    };

} // namespace finances::accounts::models

namespace utils::db {

    template <>
    std::vector<finances::accounts::models::Account>
    utils::db::ModelManager<finances::accounts::models::Account>::_all(pqxx::work&);

    template <>
    template <>
    std::vector<finances::accounts::models::Account>
    utils::db::ModelManager<finances::accounts::models::Account>::_filter_by_fk<finances::accounts::models::Custodian>(
        pqxx::work&, const decltype(finances::accounts::models::Custodian::id)& id);

    template <>
    template <>
    std::vector<finances::accounts::models::Account>
    utils::db::ModelManager<finances::accounts::models::Account>::_filter_by_fk<
        finances::accounts::models::AccountType>(pqxx::work&,
                                                 const decltype(finances::accounts::models::AccountType::id)& id);

} // namespace utils::db

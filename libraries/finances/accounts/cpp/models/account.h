#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"
#include "libraries/utils/cpp/libpqxx/orm/manager.h"

#include "custodian.h"
#include "hierarchy_tree.h"
#include "model_manager.hpp"
#include "types/ccy.h"

namespace finances::accounts::models {

    struct Account {
        utils::db::Id id;
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
    ModelManager<finances::accounts::models::Account>::_all(pqxx::work&);

    template <>
    template <>
    std::vector<finances::accounts::models::Account>
    ModelManager<finances::accounts::models::Account>::_filter_by_fk<finances::accounts::models::Custodian>(
        pqxx::work&, const decltype(finances::accounts::models::Custodian::id)& id);

    template <>
    template <>
    std::vector<finances::accounts::models::Account>
    ModelManager<finances::accounts::models::Account>::_filter_by_fk<finances::accounts::models::AccountType>(
        pqxx::work&, const decltype(finances::accounts::models::AccountType::id)& id);

    template <>
    ExpectedType<finances::accounts::models::Account, ErrorNotFound, ErrorMultipleFound>
    ModelManager<finances::accounts::models::Account>::_get(pqxx::work&,
                                                            const decltype(finances::accounts::models::Account::id)&);

} // namespace utils::db

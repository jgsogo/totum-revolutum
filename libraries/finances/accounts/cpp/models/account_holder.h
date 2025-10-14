#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/orm/manager.h"

#include "account.h"
#include "model_manager.hpp"
#include "types/id.h"

namespace finances::accounts::models {

    struct AccountHolder {
        Id id;
        std::string name;
        bool is_company;
        std::optional<std::string> photo;

        std::pair<decltype(Account::id), decltype(Account::name)> account;
        bool owns_money;
    };

} // namespace finances::accounts::models

namespace utils::db {

    template <>
    std::vector<finances::accounts::models::AccountHolder>
    ModelManager<finances::accounts::models::AccountHolder>::_all(pqxx::work&);

    template <>
    ExpectedType<finances::accounts::models::AccountHolder, ErrorNotFound, ErrorMultipleFound>
    ModelManager<finances::accounts::models::AccountHolder>::_get(
        pqxx::work&, const ModelData<finances::accounts::models::AccountHolder>::Id&);

    template <>
    template <>
    std::vector<finances::accounts::models::AccountHolder>
    utils::db::ModelManager<finances::accounts::models::AccountHolder>::_filter_by_fk<
        finances::accounts::models::Account>(pqxx::work&, const decltype(finances::accounts::models::Account::id)& id);

} // namespace utils::db

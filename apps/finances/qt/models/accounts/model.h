#pragma once

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/account_holder.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"

struct AccountModel {
    using Id = decltype(finances::accounts::models::Account::id);
    // AccountModel(AccountModel&& other) = default;
    // AccountModel& operator=(AccountModel&& other) = default;

    const Id id;

    finances::accounts::models::Account account;
    std::optional<finances::accounts::models::Snapshot> last_snapshot;
    std::vector<std::pair<finances::accounts::models::AccountHolder, finances::accounts::models::AccountHolderRole>>
        holders;
};

namespace utils::db {

    template <> ExpectedType<std::vector<AccountModel>, DatabaseError> ModelManager<AccountModel>::all();

    // template <> std::vector<AccountModel> ModelManager<AccountModel>::get_all(pqxx::work&);

    // template <>
    // ExpectedType<AccountModel, ErrorNotFound, ErrorMultipleFound>
    // Model<AccountModel>::get(pqxx::work&, const Model<AccountModel>::Id&);

} // namespace utils::db

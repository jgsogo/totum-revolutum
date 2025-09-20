#pragma once

#include <optional>
#include <string>

#include "account_type.h"
#include "ccy.h"
#include "custodian.h"
#include "id.h"
#include "model_manager.hpp"

namespace finances::accounts::models {

    struct Account {
        Id id;
        std::string name;
        std::optional<std::string> description;
        std::optional<std::string> identifier;
        Ccy ccy;
        // pub open: chrono::NaiveDate,
        // pub close: Option<chrono::NaiveDate>,
        std::pair<decltype(AccountType::id), decltype(AccountType::name)> type;
        std::pair<decltype(Custodian::id), decltype(Custodian::name)> custodian;
        bool is_numerable;
    };

    class AccountManager : public ModelManager<Account> {
      public:
        AccountManager(utils::db::ConnectionPool& pool);
    };

    template <> tl::expected<std::vector<Account>, Error> ModelManager<Account>::all();

} // namespace finances::accounts::models

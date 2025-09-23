#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/date.h"

#include "account_type.h"
#include "custodian.h"
#include "model_manager.hpp"
#include "snapshot.h"
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

    class AccountManager : public ModelManager<Account> {
      public:
        AccountManager(utils::libpqxx::ConnectionPool& pool);

        tl::expected<std::optional<Snapshot>, Error> get_last_snapshot(decltype(Account::id) account_id) const;

        tl::expected<std::vector<std::optional<Snapshot>>, Error>
        get_last_snapshots(const std::vector<decltype(Account::id)>& account_ids) const;
    };

    template <> tl::expected<std::vector<Account>, Error> ModelManager<Account>::all();

} // namespace finances::accounts::models

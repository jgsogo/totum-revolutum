#include "model.h"

#include <spdlog/spdlog.h>

namespace utils::db {

    template <> std::vector<AccountModel> utils::db::Model<AccountModel>::get_all(pqxx::work& tx) {
        auto all_accounts = utils::db::Model<finances::accounts::models::Account>::get_all(tx);

        std::vector<AccountModel> ret;
        std::transform(all_accounts.begin(), all_accounts.end(), std::back_inserter(ret),
                       [](const auto& acc) -> AccountModel {
                           return {
                               .id = acc.id, .account = acc,
                               // .last_snapshot = ,
                               // .holders =
                           };
                       });
        return ret;
    }

} // namespace utils::db

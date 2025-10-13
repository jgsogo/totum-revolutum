#include "model.h"

#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

namespace utils::db {

    template <> ExpectedType<std::vector<AccountModel>, DatabaseError> ModelManager<AccountModel>::all() {

        // Get all accounts
        auto accounts_manager = ModelData<Account>::Manager{pool};
        auto all_accounts = accounts_manager.all();
        if (!all_accounts) {
            return tl::unexpected{all_accounts.error()};
        }

        std::vector<AccountModel> ret;
        for (auto&& acc : all_accounts.value()) {
            // Get the last snapshot for this accounts
            auto snapshots_manager = SnapshotManager{pool};
            auto last_snapshot = snapshots_manager.get_last_snapshot(acc.id);
            if (!last_snapshot) {
                return tl::unexpected{last_snapshot.error()};
            }

            // Get the holders for this account
            AccountHolderManager holders_manager{pool};
            auto holders = holders_manager.all_for_account(acc.id);
            if (!holders) {
                return tl::unexpected{holders.error()};
            }

            // Create the AccountModel, we have all the information we need
            ret.emplace_back(AccountModel{.id = acc.id,
                                          .account = std::move(acc),
                                          .last_snapshot = std::move(last_snapshot.value()),
                                          .holders = std::move(holders.value())});
        }

        return ret;
    }

} // namespace utils::db

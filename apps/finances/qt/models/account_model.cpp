#include "account_model.h"

#include <spdlog/spdlog.h>

using namespace finances::accounts::models;
using namespace finances::investments::models;

std::optional<std::reference_wrapper<const utils::libpqxx::Date>> AccountModel::last_snapshot_date() const {
    if (last_snapshot) {
        const utils::libpqxx::Date& date = std::visit(
            [](const auto& arg) -> const utils::libpqxx::Date& {
                using T = std::decay_t<decltype(arg)>;
                if constexpr (std::is_same_v<T, Snapshot>) {
                    return arg.date_value;
                } else if constexpr (std::is_same_v<T, SnapshotNumerable>) {
                    return arg.snapshot.date_value;
                } else {
                    static_assert(false, "non-exhaustive visitor!");
                }
            },
            last_snapshot.value());
        return {date};
    } else {
        return std::nullopt;
    }
}

std::optional<std::reference_wrapper<const Money>> AccountModel::last_snapshot_amount() const {
    if (last_snapshot) {
        const Money& money_amount = std::visit(
            [](const auto& arg) -> const Money& {
                using T = std::decay_t<decltype(arg)>;
                if constexpr (std::is_same_v<T, Snapshot>) {
                    return arg.amount;
                } else if constexpr (std::is_same_v<T, SnapshotNumerable>) {
                    return arg.snapshot.amount;
                } else {
                    static_assert(false, "non-exhaustive visitor!");
                }
            },
            last_snapshot.value());
        return {money_amount};
    } else {
        return std::nullopt;
    }
}

namespace utils::db {

    namespace {

        ExpectedType<AccountModel, DatabaseError> get_account_model(utils::libpqxx::ConnectionPool& pool,
                                                                    Account&& account) {
            decltype(AccountModel::last_snapshot) last_snapshot = std::nullopt;

            // Get the last snapshot for this account
            if (account.is_numerable) {
                auto snapshot_numerable_manager = SnapshotNumerableManager{pool};
                auto last_snapshot_expected = snapshot_numerable_manager.get_last_snapshot(account.id);
                if (!last_snapshot_expected) {
                    return tl::unexpected{last_snapshot_expected.error()};
                }
                last_snapshot = std::move(last_snapshot_expected.value());
            } else {
                auto snapshots_manager = SnapshotManager{pool};
                auto last_snapshot_expected = snapshots_manager.get_last_snapshot(account.id);
                if (!last_snapshot_expected) {
                    return tl::unexpected{last_snapshot_expected.error()};
                }
                last_snapshot = std::move(last_snapshot_expected.value());
            }

            // Get the holders for this account
            utils::db::ModelManager<AccountHolderWithRoles> holders_manager{pool};
            auto holders = holders_manager.filter_by_fk(account);
            if (!holders) {
                return tl::unexpected{holders.error()};
            }

            // Breadcrumb
            AccountTypeManager acctype_manager{pool};
            auto breadcrumb = acctype_manager.breadcrumb(account.type.first);

            std::string breadcrumb_str;
            if (!breadcrumb) {
                SPDLOG_WARN("Error retrieving acctype breadcrumb for account {}: {}", account.id, breadcrumb.error());
                // TODO: Communicate error to user
                breadcrumb_str = account.type.second;
            } else {
                for (auto it : breadcrumb.value()) {
                    breadcrumb_str.append(it.second);
                    breadcrumb_str.append(" > ");
                }
                breadcrumb_str.append(account.type.second);
            }

            // Create the AccountModel, we have all the information we need
            return AccountModel{.id = account.id,
                                .account = std::move(account),
                                .last_snapshot = std::move(last_snapshot),
                                .holders = std::move(holders.value()),
                                .account_type_breadcrumb = breadcrumb_str};
        }

    } // namespace

    template <> ExpectedType<std::vector<AccountModel>, DatabaseError> ModelManager<AccountModel>::all() {

        // Get all accounts
        auto accounts_manager = ModelData<Account>::Manager{pool};
        auto all_accounts = accounts_manager.all();
        if (!all_accounts) {
            return tl::unexpected{all_accounts.error()};
        }

        std::vector<AccountModel> ret;
        for (auto&& acc : all_accounts.value()) {
            auto expected = get_account_model(pool, std::move(acc));
            if (!expected) {
                return tl::unexpected{expected.error()};
            }
            ret.emplace_back(std::move(expected.value()));
        }

        return ret;
    }

    template <>
    ExpectedType<AccountModel, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<AccountModel>::get(const ModelData<AccountModel>::Id& id) {
        auto accounts_manager = ModelData<Account>::Manager{pool};
        auto account_expected = accounts_manager.get(id);
        if (!account_expected) {
            return tl::unexpected{account_expected.error()};
        }

        finances::accounts::models::Account account = std::move(account_expected.value());
        return get_account_model(pool, std::move(account));
    }

} // namespace utils::db

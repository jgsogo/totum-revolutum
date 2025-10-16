#include "table.h"

AccountsTable::AccountsTable(utils::libpqxx::ConnectionPool& pool, QObject* parent)
    : utils::qt::models::TableModel<AccountModel, AccountColumns>{pool, parent} {}

void AccountsTable::_refresh_all() {
    SPDLOG_DEBUG("AccountsTable::_refresh_all");
    utils::qt::models::TableModel<AccountModel, AccountColumns>::_refresh_all();

    // Populate breadcrumb
    utils::db::AccountTypeManager acctype_manager{pool};
    for (auto& item : items) {
        auto breadcrumb = acctype_manager.breadcrumb(item.account.type.first);
        std::string breadcrumb_str;
        if (!breadcrumb) {
            SPDLOG_WARN("Error retrieving breadcrumb for account {}: {}", item.id, breadcrumb.error());
            // TODO: Communicate error to user
            breadcrumb_str = item.account.type.second;
        } else {
            for (auto it : breadcrumb.value()) {
                breadcrumb_str.append(it.second);
                breadcrumb_str.append(" > ");
            }
            breadcrumb_str.append(item.account.type.second);
        }
        item.account_type_breadcrumb = breadcrumb_str;
    }
};

void AccountsTable::_refresh_one(const utils::db::ModelData<AccountModel>::Id& id, int row) {
    SPDLOG_DEBUG("AccountsTable::_refresh_one(id={}, row={})", id, row);

    // For a given row, the only thing that can change is the last_snapshot
    utils::db::SnapshotManager manager{pool};
    auto last_snapshot = manager.get_last_snapshot(id);
    if (!last_snapshot) {
        SPDLOG_ERROR("Error retrieving last snapshot for snapshot {}: {}", id, last_snapshot.error());
        // TODO: Communicate error to user
        return;
    }

    this->items.at(row).last_snapshot = std::move(last_snapshot.value());

    // Emit a signal to notify that the snapshot column has been modified
    QVector<int> roles = {Qt::DisplayRole};
    QModelIndex topLeft = this->createIndex(row, magic_enum::enum_integer(AccountColumns::SNAPSHOT));
    emit dataChanged(topLeft, topLeft, roles);
}

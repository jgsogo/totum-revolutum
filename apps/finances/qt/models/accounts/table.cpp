#include "table.h"

AccountsTable::AccountsTable(utils::libpqxx::ConnectionPool& pool, QObject* parent)
    : utils::qt::models::TableModel<AccountModel, AccountColumns>{pool, parent} {}

void AccountsTable::_refresh_one(const AccountModel::Id& id, int row) {
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
namespace utils::qt::models {

    // template <>
    // QVariant
    // DataDispatcher<AccountModel, AccountColumns, Qt::DisplayRole>::data(const TableModel<AccountModel,
    // AccountColumns>&,
    //                                                                     const AccountModel&, AccountColumns) {
    //     return QVariant{QString::fromStdString("lol")};
    // }

} // namespace utils::qt::models

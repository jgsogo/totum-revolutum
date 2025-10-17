#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "apps/finances/qt/models/accounts/model.h"

template <typename Columns>
class AccountsTableModel final : public utils::qt::models::TableModel<AccountModel, Columns> {
  public:
    AccountsTableModel(utils::libpqxx::ConnectionPool& pool, QObject* parent = nullptr)
        : utils::qt::models::TableModel<AccountModel, Columns>{pool, parent} {};

  protected:
    void _refresh_one(const utils::db::ModelData<AccountModel>::Id& id, int row) override final {
        SPDLOG_DEBUG("AccountsTableModel::_refresh_one(id={}, row={})", id, row);

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
};

#include "accounts_table_filter.h"

#include <QDate>
#include <magic_enum/magic_enum.hpp>
#include <spdlog/spdlog.h>

#include "apps/finances/qt/models/accounts_table.h"

AccountsTableFilterProxyModel::AccountsTableFilterProxyModel(Qt::CheckState showClosedAccounts,
                                                             Qt::CheckState showOthersAccounts, QObject* parent)
    : QSortFilterProxyModel(parent), _showClosedAccounts{showClosedAccounts},
      _showOthersAccounts{showOthersAccounts} {};

void AccountsTableFilterProxyModel::showClosedAccounts(Qt::CheckState state) {
    SPDLOG_DEBUG("AccountsTableFilterProxyModel::showClosedAccounts(state={})",
                 static_cast<int>(state)); // TODO: Implement a utils/cpp for qt

    beginFilterChange();
    _showClosedAccounts = state;
    invalidateRowsFilter();
}

void AccountsTableFilterProxyModel::showOthersAccounts(Qt::CheckState state) {
    SPDLOG_DEBUG("AccountsTableFilterProxyModel::showOthersAccounts(state={})",
                 static_cast<int>(state)); // TODO: Implement a utils/cpp for qt

    beginFilterChange();
    _showOthersAccounts = state;
    invalidateRowsFilter();
}

bool AccountsTableFilterProxyModel::filterAcceptsRow(int sourceRow, const QModelIndex& sourceParent) const {
    if (this->QSortFilterProxyModel::filterAcceptsRow(sourceRow, sourceParent)) {
        // Filter based on showClosedAccounts
        if (_showClosedAccounts == Qt::Unchecked) {
            // FIXME: Use some account.isClosed() helper method
            QModelIndex idx_close = sourceModel()->index(
                sourceRow, magic_enum::enum_integer(AccountTableModel::Column::CLOSE), sourceParent);
            QVariant close_value = sourceModel()->data(idx_close);
            if (!close_value.isNull()) {
                if (close_value.toDate() < QDate::currentDate()) {
                    return false;
                }
            }
        }
        // Filter base on showOthersAccounts
        // TODO: Not implemented
        return true;
    }
    return false;
};

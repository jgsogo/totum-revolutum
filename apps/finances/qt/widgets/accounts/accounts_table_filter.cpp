#include "accounts_table_filter.h"

#include <QDate>
#include <magic_enum/magic_enum.hpp>
#include <spdlog/spdlog.h>

#include "apps/finances/qt/models/accounts_table.h"

AccountsTableFilterProxyModel::AccountsTableFilterProxyModel(
    std::optional<finances::accounts::models::AccountHolder> me_, const AccountTableModel* model_,
    Qt::CheckState showClosedAccounts, Qt::CheckState showOthersAccounts, QObject* parent)
    : QSortFilterProxyModel(parent), model{model_}, me{me_}, _showClosedAccounts{showClosedAccounts},
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
    SPDLOG_DEBUG("AccountsTableFilterProxyModel::filterAcceptsRow(sourceRow={})", sourceRow);

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
        if (me.has_value() && _showOthersAccounts == Qt::Unchecked) {
            // SPDLOG_TRACE("1");
            // SPDLOG_ERROR("mapToSource -- row={}", this->createSourceIndex(sourceRow, 1, nullptr).row());
            // auto idx_model = this->mapToSource(this->createSourceIndex(sourceRow, 1, nullptr));
            // SPDLOG_TRACE("2");
            const auto& holders = this->model->get_holders(sourceRow);
            auto it = std::find_if(holders.begin(), holders.end(), [this](const auto& acc_holder) {
                return acc_holder.second.owns_money && (acc_holder.first.id == me->id);
            });
            return it != holders.end();
        }
        return true;
    }
    return false;
};

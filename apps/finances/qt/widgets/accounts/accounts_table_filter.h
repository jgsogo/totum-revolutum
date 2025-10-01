#pragma once

#include <QSortFilterProxyModel>

#include "apps/finances/qt/models/accounts_table.h"
#include "libraries/finances/accounts/cpp/models/account_holder.h"

class AccountsTableFilterProxyModel : public QSortFilterProxyModel {
    Q_OBJECT

  public:
    AccountsTableFilterProxyModel(std::optional<finances::accounts::models::AccountHolder> me,
                                  const AccountTableModel* model, Qt::CheckState showClosedAccounts,
                                  Qt::CheckState showOthersAccounts, QObject* parent = nullptr);

  public slots:
    void showClosedAccounts(Qt::CheckState state);
    void showOthersAccounts(Qt::CheckState state);

  protected:
    bool filterAcceptsRow(int sourceRow, const QModelIndex& sourceParent) const override;

  private:
    const AccountTableModel* model;
    std::optional<finances::accounts::models::AccountHolder> me;
    Qt::CheckState _showClosedAccounts;
    Qt::CheckState _showOthersAccounts;
};

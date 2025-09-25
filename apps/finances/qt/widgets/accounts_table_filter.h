#pragma once

#include <QSortFilterProxyModel>

class AccountsTableFilterProxyModel : public QSortFilterProxyModel {
    Q_OBJECT

  public:
    AccountsTableFilterProxyModel(Qt::CheckState showClosedAccounts, Qt::CheckState showOthersAccounts,
                                  QObject* parent = nullptr);

  public slots:
    void showClosedAccounts(Qt::CheckState state);
    void showOthersAccounts(Qt::CheckState state);

  protected:
    bool filterAcceptsRow(int sourceRow, const QModelIndex& sourceParent) const override;

  private:
    Qt::CheckState _showClosedAccounts;
    Qt::CheckState _showOthersAccounts;
};

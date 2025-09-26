#pragma once

#include <QWidget>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "apps/finances/qt/models/account_related.h"

class AccountDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountDetailWidget(const finances::accounts::models::Account&, AccountSnapshotsModel* snapshots_model,
                                 AccountMovementsModel* movements_model, QWidget* parent = nullptr);

  private:
    const finances::accounts::models::Account& account;
    // AccountsTableFilterProxyModel* sort_filter;
    // AccountTableModel* model;
};

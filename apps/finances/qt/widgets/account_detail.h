#pragma once

#include <QWidget>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "account_add_snapshot.h"
#include "apps/finances/qt/models/account_related.h"

class AccountDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountDetailWidget(const finances::accounts::models::Account&, AccountRelatedModelBase* snapshots_model,
                                 AccountRelatedModelBase* movements_model, QWidget* parent = nullptr);

  public slots:
    void on_new_snapshot(Snapshot);

  private:
    const finances::accounts::models::Account& account;
    // AccountsTableFilterProxyModel* sort_filter;
    // AccountTableModel* model;
};

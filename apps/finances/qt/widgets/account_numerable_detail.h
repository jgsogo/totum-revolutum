#pragma once

#include <QWidget>

#include "apps/finances/qt/metatypes/snapshot_numerable.h"
#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

class AccountNumerableDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool,
                                          const finances::accounts::models::Account&, QWidget* parent = nullptr);

  public slots:
    void on_new_snapshot(SnapshotNumerable);
    // void on_new_snapshot(SnapshotNumerable);

  signals:
    void snapshot_added(finances::accounts::models::Id account_id);

  private:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
    // AccountsTableFilterProxyModel* sort_filter;
    // AccountTableModel* model;
};

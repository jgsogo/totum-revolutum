#pragma once

#include <QWidget>

#include "apps/finances/qt/metatypes/snapshot_non_numerable.h"
#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

class AccountNonNumerableDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountNonNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool,
                                             const finances::accounts::models::Account&, QWidget* parent = nullptr);

  public slots:
    void on_new_snapshot(SnapshotNonNumerable);
    // void on_new_snapshot(SnapshotNumerable);

  signals:
    void snapshot_added(finances::accounts::models::Id account_id);

  private:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
    // AccountsTableFilterProxyModel* sort_filter;
    // AccountTableModel* model;
};

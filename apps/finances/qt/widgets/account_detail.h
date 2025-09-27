#pragma once

#include <QWidget>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "account_add_snapshot.h"
#include "apps/finances/qt/models/account_related.h"

class AccountDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountDetailWidget(utils::libpqxx::ConnectionPool& pool, const finances::accounts::models::Account&,
                                 QWidget* parent = nullptr);

  public slots:
    void on_new_snapshot(Snapshot2Decs);

  signals:
    void snapshot_added(finances::accounts::models::Id account_id);

  private:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
    // AccountsTableFilterProxyModel* sort_filter;
    // AccountTableModel* model;
};

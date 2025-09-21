#pragma once

#include <QSortFilterProxyModel>
#include <QTimer>
#include <QWidget>

#include "libraries/finances/accounts/cpp/models/id.h"
#include "libraries/utils/cpp/db/connection_pool.h"

#include "apps/finances/qt/models/accounts_table.h"

class AccountsTableWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountsTableWidget(utils::db::ConnectionPool& pool, QWidget* parent = nullptr,
                                 Qt::WindowFlags f = Qt::WindowFlags());

  public slots:
    void refresh_all_accounts();
    void refresh_account(finances::accounts::models::Id);

  private:
    utils::db::ConnectionPool& pool;
    QSortFilterProxyModel* sort_filter;
    AccountTableModel* model;
};

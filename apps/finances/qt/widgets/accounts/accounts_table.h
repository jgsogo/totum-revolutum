#pragma once

#include <QTableWidget>
#include <QWidget>

#include "apps/finances/qt/models/accounts_table.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "accounts_table_filter.h"

class AccountsTableWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountsTableWidget(utils::libpqxx::ConnectionPool& pool, AccountTableModel* model,
                                 QWidget* parent = nullptr);

  private slots:
    void onDoubleClicked(const QModelIndex& index);
    void onPressed(const QModelIndex& index);

  signals:
    void accountDoubleClicked(finances::accounts::models::Id account_id);

  private:
    utils::libpqxx::ConnectionPool& pool;
    AccountTableModel* model;
    AccountsTableFilterProxyModel* sort_filter;
};

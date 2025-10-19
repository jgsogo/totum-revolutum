#pragma once

#include <QTableWidget>
#include <QWidget>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "apps/finances/qt/table_models/accounts.h"
#include "apps/finances/qt/tables/accounts.h"

#include "accounts_table_filter.h"

class AccountsTableWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountsTableWidget(utils::libpqxx::ConnectionPool& pool, AccountsTableModel<AccountColumns>* model,
                                 std::optional<finances::accounts::models::AccountHolder> me,
                                 QWidget* parent = nullptr);

  private slots:
    void onDoubleClicked(const QModelIndex& index);
    void onPressed(const QModelIndex& index);

  signals:
    void accountDoubleClicked(utils::db::Id account_id);

  private:
    utils::libpqxx::ConnectionPool& pool;
    AccountsTableModel<AccountColumns>* model;
    AccountsTableFilterProxyModel* sort_filter;
};

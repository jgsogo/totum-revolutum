#pragma once

#include <QTableWidget>
#include <QWidget>

#include "apps/finances/qt/models/accounts_table.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "accounts_table_filter.h"

class AccountsTableWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountsTableWidget(AccountTableModel* model, QWidget* parent = nullptr);

  private slots:
    void onDoubleClicked(const QModelIndex& index);

  signals:
    void accountDoubleClicked(finances::accounts::models::Id account_id);

  private:
    AccountsTableFilterProxyModel* sort_filter;
};

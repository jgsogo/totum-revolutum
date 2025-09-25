#pragma once

#include <QWidget>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "apps/finances/qt/models/accounts_table.h"
#include "apps/finances/qt/widgets/accounts_table_filter.h"

class AccountsTableWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountsTableWidget(utils::libpqxx::ConnectionPool& pool, QWidget* parent = nullptr,
                                 Qt::WindowFlags f = Qt::WindowFlags());

  private:
    AccountsTableFilterProxyModel* sort_filter;
    AccountTableModel* model;
};

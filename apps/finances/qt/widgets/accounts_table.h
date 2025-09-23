#pragma once

#include <QSortFilterProxyModel>
#include <QWidget>

#include "libraries/utils/cpp/db/connection_pool.h"

#include "apps/finances/qt/models/accounts_table.h"

class AccountsTableWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountsTableWidget(utils::db::ConnectionPool& pool, QWidget* parent = nullptr,
                                 Qt::WindowFlags f = Qt::WindowFlags());

  private:
    QSortFilterProxyModel* sort_filter;
    AccountTableModel* model;
};

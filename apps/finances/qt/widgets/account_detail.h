#pragma once

#include <QWidget>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"

class AccountDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountDetailWidget(utils::libpqxx::ConnectionPool& pool, QWidget* parent = nullptr,
                                 Qt::WindowFlags f = Qt::WindowFlags());

  private:
    // AccountsTableFilterProxyModel* sort_filter;
    // AccountTableModel* model;
};

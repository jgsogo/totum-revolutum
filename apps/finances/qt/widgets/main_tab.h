#pragma once

#include <QTabWidget>

#include "libraries/finances/accounts/cpp/models/types/id.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

class MainTabWidget : public QTabWidget {
    Q_OBJECT

  public:
    MainTabWidget(utils::libpqxx::ConnectionPool& pool, QWidget* parent = nullptr);

  private slots:
    void addTabAccount(finances::accounts::models::Id account_id);

  private:
    int _all_accounts_idx;
    utils::libpqxx::ConnectionPool& pool;
};

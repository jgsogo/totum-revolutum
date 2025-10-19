#pragma once

#include <QWidget>

#include "apps/finances/qt/models/account_model.h"

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

class AccountDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountDetailWidget(utils::libpqxx::ConnectionPool& pool, const AccountModel&, QWidget* parent = nullptr);

  private slots:
    void on_new_snapshot(utils::db::Id account_id);

  signals:
    void snapshot_added(utils::db::Id account_id);

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const AccountModel& account;
};

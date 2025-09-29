#pragma once

#include <QWidget>

#include "apps/finances/qt/metatypes/snapshot_numerable.h"
#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

class AccountDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountDetailWidget(utils::libpqxx::ConnectionPool& pool, const finances::accounts::models::Account&,
                                 QWidget* parent = nullptr);

  signals:
    void snapshot_added(finances::accounts::models::Id account_id);

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
};

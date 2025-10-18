#pragma once

#include <QWidget>

#include "apps/finances/qt/models/account_model.h"
#include "apps/finances/qt/models/movement_type.h"
#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

class AccountDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountDetailWidget(utils::libpqxx::ConnectionPool& pool, const AccountModel&,
                                 const MovementTypeTableModel* movtype_model, QWidget* parent = nullptr);

  signals:
    void snapshot_added(utils::db::Id account_id);

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const AccountModel& account;
    const MovementTypeTableModel* movtype_model;
};

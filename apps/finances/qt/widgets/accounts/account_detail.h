#pragma once

#include <QWidget>

#include "apps/finances/qt/metatypes/snapshot_numerable.h"
#include "apps/finances/qt/models/movement_type.h"
#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

class AccountDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountDetailWidget(utils::libpqxx::ConnectionPool& pool, const finances::accounts::models::Account&,
                                 const MovementTypeTableModel* movtype_model, QWidget* parent = nullptr);

  signals:
    void snapshot_added(finances::accounts::models::Id account_id);

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
    const MovementTypeTableModel* movtype_model;
};

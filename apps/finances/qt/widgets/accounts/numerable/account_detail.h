#pragma once

#include "apps/finances/qt/widgets/accounts/account_detail.h"

class AccountNumerableDetailWidget : public AccountDetailWidget {
    Q_OBJECT
  public:
    explicit AccountNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool, const AccountModel&,
                                          const MovementTypeTableModel* movtype_model, QWidget* parent = nullptr);

  private slots:
    void on_new_snapshot(utils::db::Id account_id);
    // void on_new_snapshot(SnapshotNumerable);
};

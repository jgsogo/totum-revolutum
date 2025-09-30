#pragma once

#include "apps/finances/qt/metatypes/snapshot_numerable.h"
#include "apps/finances/qt/widgets/accounts/account_detail.h"

class AccountNumerableDetailWidget : public AccountDetailWidget {
    Q_OBJECT
  public:
    explicit AccountNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool,
                                          const finances::accounts::models::Account&,
                                          MovementTypeTableModel* movtype_model, QWidget* parent = nullptr);

  public slots:
    void on_new_snapshot(SnapshotNumerable);
    // void on_new_snapshot(SnapshotNumerable);
};

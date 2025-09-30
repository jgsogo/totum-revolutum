#pragma once

#include "apps/finances/qt/metatypes/snapshot_non_numerable.h"
#include "apps/finances/qt/widgets/accounts/account_detail.h"

class AccountNonNumerableDetailWidget : public AccountDetailWidget {
    Q_OBJECT
  public:
    explicit AccountNonNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool,
                                             const finances::accounts::models::Account&,
                                             const MovementTypeTableModel* movtype_model, QWidget* parent = nullptr);

  public slots:
    void on_new_snapshot(SnapshotNonNumerable);
};

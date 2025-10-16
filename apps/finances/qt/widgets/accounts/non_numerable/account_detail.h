#pragma once

#include "apps/finances/qt/widgets/accounts/account_detail.h"

class AccountNonNumerableDetailWidget : public AccountDetailWidget {
    Q_OBJECT
  public:
    explicit AccountNonNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool,
                                             const finances::accounts::models::Account&,
                                             const MovementTypeTableModel* movtype_model, QWidget* parent = nullptr);

  private slots:
    void on_new_snapshot(utils::db::Id account_id);
};

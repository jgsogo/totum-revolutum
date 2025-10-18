#pragma once

#include "apps/finances/qt/widgets/accounts/account_detail.h"

class AccountNonNumerableDetailWidget : public AccountDetailWidget {
    Q_OBJECT
  public:
    explicit AccountNonNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool, const AccountModel&,
                                             QWidget* parent = nullptr);

  private slots:
    void on_new_snapshot(utils::db::Id account_id);
};

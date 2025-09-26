#pragma once

#include "account_related.h"

class AccountRelatedMovementsModel : public AccountRelatedModel<finances::accounts::models::Movement, MovementColumn> {
    Q_OBJECT
  public:
  public:
    using AccountRelatedModel::AccountRelatedModel;

  private slots:
    void fetch_all_movements();
};

template <>
QVariant DataDispatcher<finances::accounts::models::Movement, MovementColumn, Qt::DisplayRole>::data(
    const finances::accounts::models::Account&, const finances::accounts::models::Movement&, MovementColumn);

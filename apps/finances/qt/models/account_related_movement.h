#pragma once

#include "account_related.h"

enum class MovementColumn {
    ID = 0,
    DATE_VALUE = 1,
    MOVE_TYPE = 2,
    TRANSACTION = 3,
    DIRECTION = 4,
    AMOUNT = 5,
};

class AccountRelatedMovementsModel : public AccountRelatedModel<finances::accounts::models::Movement, MovementColumn> {
    Q_OBJECT
  public:
  public:
    using AccountRelatedModel::AccountRelatedModel;

  private slots:
    void fetch_all_movements();
};

template <>
QVariant AccountRelatedModel<finances::accounts::models::Movement, MovementColumn>::data_display_role(
    MovementColumn column, const finances::accounts::models::Movement& item) const;

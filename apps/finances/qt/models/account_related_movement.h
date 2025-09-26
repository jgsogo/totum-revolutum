#pragma once

#include "account_related.h"

class AccountRelatedMovementsModel : public AccountRelatedModel<finances::accounts::models::Movement> {
    Q_OBJECT
  public:
    // enum class Column {
    //     ID = 0,
    //     DATE_VALUE = 1,
    //     AMOUNT = 2,
    //     TRANSACTION = 3,
    //     MOVE_TYPE = 4,
    //     DIRECTION = 5,
    // };
  public:
    using AccountRelatedModel::AccountRelatedModel;

  private slots:
    void fetch_all_movements();
};

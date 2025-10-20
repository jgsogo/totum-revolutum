

#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "apps/finances/qt/models/movement_model.h"
#include "libraries/finances/accounts/cpp/models/account.h"

template <typename Columns>
using MovementsForAccountTableModel =
    utils::qt::models::FilteredTableModel<finances::accounts::models::Account, MovementModel, Columns>;

template <typename Columns>
using MovementsForTransactionTableModel =
    utils::qt::models::FilteredTableModel<finances::accounts::models::Transaction, MovementModel, Columns>;

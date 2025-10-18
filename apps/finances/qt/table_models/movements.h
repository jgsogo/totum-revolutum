

#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "apps/finances/qt/models/account_model.h"
#include "apps/finances/qt/models/movement_model.h"

template <typename Columns>
using MovementsTableModel = utils::qt::models::FilteredTableModel<AccountModel, MovementModel, Columns>;

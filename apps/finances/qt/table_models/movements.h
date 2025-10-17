

#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "apps/finances/qt/models/accounts/model.h"
#include "apps/finances/qt/models/movements/model.h"

template <typename Columns>
using MovementsTableModel = utils::qt::models::FilteredTableModel<AccountModel, MovementModel, Columns>;

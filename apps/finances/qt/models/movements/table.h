#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "model.h"

enum class WIPColumns {
    ID = 0,
};

using MovementsTable = utils::qt::models::FilteredTableModel<AccountModel, MovementModel, WIPColumns>;

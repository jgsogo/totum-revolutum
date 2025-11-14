

#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "apps/finances/qt/models/movtype_model.h"

template <typename Columns> using MovementTypesTableModel = utils::qt::models::TableModel<MovementTypeModel, Columns>;

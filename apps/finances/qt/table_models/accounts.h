#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "apps/finances/qt/models/account_model.h"

template <typename Columns> using AccountsTableModel = utils::qt::models::TableModel<AccountModel, Columns>;

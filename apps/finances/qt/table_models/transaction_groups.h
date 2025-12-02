#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "libraries/finances/accounts/cpp/models/transaction.h"

template <typename Columns>
using TransactionGroupTableModel = utils::qt::models::TableModel<finances::accounts::models::TransactionGroup, Columns>;

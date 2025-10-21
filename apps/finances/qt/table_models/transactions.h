#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/transaction.h"

template <typename Columns>
using TransactionsForAccountTableModel =
    utils::qt::models::FilteredTableModel<finances::accounts::models::Account, finances::accounts::models::Transaction,
                                          Columns>;

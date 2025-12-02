#pragma once

#include "libraries/utils/cpp/qt/models/data_dispatcher.h"

#include "libraries/finances/accounts/cpp/models/transaction.h"

enum class TransactionGroupColumns {
    ID = 0,
    NAME = 1,
    DESCRIPTION = 2,
    START = 3,
    END = 4,
    // CADENCE = 5,
};

namespace utils::qt::models {

    template <>
    QVariant
    DataDispatcher<finances::accounts::models::TransactionGroup, TransactionGroupColumns, Qt::DisplayRole>::data(
        const finances::accounts::models::TransactionGroup&, TransactionGroupColumns);

} // namespace utils::qt::models

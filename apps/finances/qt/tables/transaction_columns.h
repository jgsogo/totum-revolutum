#pragma once

#include "libraries/utils/cpp/qt/models/data_dispatcher.h"

#include "libraries/finances/accounts/cpp/models/transaction.h"

enum class TransactionColumns {
    ID = 0,
    NAME = 1,
    DESCRIPTION = 2,
    GROUP_ID = 3,
    GROUP_NAME = 4,
    // GROUP_CADENCE = 5,
};

namespace utils::qt::models {

    template <>
    QVariant DataDispatcher<finances::accounts::models::Transaction, TransactionColumns, Qt::DisplayRole>::data(
        const finances::accounts::models::Transaction&, TransactionColumns);

    // template <>
    // QVariant DataDispatcher<AccountModel, TransactionColumns, Qt::FontRole>::data(const AccountModel&,
    // AccountColumns);

    // template <>
    // QVariant DataDispatcher<AccountModel, TransactionColumns, Qt::ForegroundRole>::data(const AccountModel&,
    //                                                                                 AccountColumns);

    // template <>
    // QVariant DataDispatcher<AccountModel, TransactionColumns, Qt::TextAlignmentRole>::data(const AccountModel&,
    //                                                                                    AccountColumns);
} // namespace utils::qt::models

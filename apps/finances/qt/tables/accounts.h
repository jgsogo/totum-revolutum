#pragma once

#include "libraries/utils/cpp/qt/models/data_dispatcher.h"

#include "apps/finances/qt/models/account_model.h"

enum class AccountColumns {
    ID = 0,
    CUSTODIAN = 1,
    NAME = 2,
    IDENTIFIER = 3,
    TYPE = 4,
    SNAPSHOT = 5,
    OPEN = 6,
    CLOSE = 7,
    HOLDERS = 8,
};

namespace utils::qt::models {

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::DisplayRole>::data(const AccountModel&, AccountColumns);

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::FontRole>::data(const AccountModel&, AccountColumns);

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::ForegroundRole>::data(const AccountModel&,
                                                                                    AccountColumns);

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::TextAlignmentRole>::data(const AccountModel&,
                                                                                       AccountColumns);
} // namespace utils::qt::models

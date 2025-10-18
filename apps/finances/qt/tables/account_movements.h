#pragma once

#include "libraries/utils/cpp/qt/models/data_dispatcher.h"

#include "libraries/finances/accounts/cpp/models/snapshot.h"
#include "libraries/finances/investments/cpp/models/snapshot_numerable.h"

#include "apps/finances/qt/models/movement_model.h"

enum class AccountMovementsColumns {
    ID = 0,
    DATE_VALUE = 1,
    MOVE_TYPE = 2,
    TRANSACTION = 3,
    DIRECTION = 4,
    AMOUNT = 5,
};

namespace utils::qt::models {

    template <>
    QVariant DataDispatcher<MovementModel, AccountMovementsColumns, Qt::DisplayRole>::data(const MovementModel&,
                                                                                           AccountMovementsColumns);

    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, AccountMovementsColumns, Qt::DisplayRole>::data(
        const finances::accounts::models::Snapshot&, AccountMovementsColumns);

    template <>
    QVariant
    DataDispatcher<finances::investments::models::SnapshotNumerable, AccountMovementsColumns, Qt::DisplayRole>::data(
        const finances::investments::models::SnapshotNumerable&, AccountMovementsColumns);

    // template <>
    // QVariant DataDispatcher<AccountModel, AccountColumns, Qt::DisplayRole>::data(const AccountModel&,
    // AccountColumns);

    // template <>
    // QVariant DataDispatcher<AccountModel, AccountColumns, Qt::FontRole>::data(const AccountModel&, AccountColumns);

    // template <>
    // QVariant DataDispatcher<AccountModel, AccountColumns, Qt::ForegroundRole>::data(const AccountModel&,
    //                                                                                 AccountColumns);

    // template <>
    // QVariant DataDispatcher<AccountModel, AccountColumns, Qt::TextAlignmentRole>::data(const AccountModel&,
    //                                                                                    AccountColumns);
} // namespace utils::qt::models

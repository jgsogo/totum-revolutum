#pragma once

#include <QFont>

#include "libraries/utils/cpp/qt/models/data_dispatcher.h"

#include "libraries/finances/accounts/cpp/models/snapshot.h"
#include "libraries/finances/investments/cpp/models/snapshot_numerable.h"

#include "apps/finances/qt/models/movement_model.h"

enum class MovementColumns {
    ID = 0,
    DATE_VALUE = 1,
    MOVE_TYPE = 2,
    TRANSACTION = 3,
    DIRECTION = 4,
    AMOUNT = 5,
    QUANTITY = 6,
    UNIT_VALUE = 7,
    ACCOUNT = 8,
    TRANSACTION_ID = 9,
    ACCOUNT_ID = 10,
};

namespace utils::qt::models {

    // Qt::DisplayRole
    template <>
    QVariant DataDispatcher<MovementModel, MovementColumns, Qt::DisplayRole>::data(const MovementModel&,
                                                                                   MovementColumns);

    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementColumns, Qt::DisplayRole>::data(
        const finances::accounts::models::Snapshot&, MovementColumns);

    template <>
    QVariant DataDispatcher<finances::investments::models::SnapshotNumerable, MovementColumns, Qt::DisplayRole>::data(
        const finances::investments::models::SnapshotNumerable&, MovementColumns);

    // Qt::FontRole
    template <typename TModel> struct DataDispatcher<TModel, MovementColumns, Qt::FontRole> {
        static QVariant data(const TModel&, MovementColumns column) {
            if ((column == MovementColumns::DATE_VALUE) || (column == MovementColumns::AMOUNT) ||
                (column == MovementColumns::QUANTITY) || (column == MovementColumns::UNIT_VALUE)) {
                return QVariant{QFont{"Andale Mono"}};
            }
            return QVariant{};
        }
    };

    // Qt::BackgroundRole
    template <>
    QVariant
    DataDispatcher<finances::investments::models::SnapshotNumerable, MovementColumns, Qt::BackgroundRole>::data(
        const finances::investments::models::SnapshotNumerable&, MovementColumns);

    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementColumns, Qt::BackgroundRole>::data(
        const finances::accounts::models::Snapshot&, MovementColumns);

} // namespace utils::qt::models

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
};

namespace utils::qt::models {

    // Qt::DisplayRole
    template <>
    template <>
    QVariant DataDispatcher<MovementModel, MovementColumns, Qt::DisplayRole>::data<finances::accounts::models::Account>(
        const MovementModel&, MovementColumns, const finances::accounts::models::Account&);

    template <>
    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementColumns, Qt::DisplayRole>::data<
        finances::accounts::models::Account>(const finances::accounts::models::Snapshot&, MovementColumns,
                                             const finances::accounts::models::Account&);

    template <>
    template <>
    QVariant DataDispatcher<finances::investments::models::SnapshotNumerable, MovementColumns, Qt::DisplayRole>::data<
        finances::accounts::models::Account>(const finances::investments::models::SnapshotNumerable&, MovementColumns,
                                             const finances::accounts::models::Account&);

    // Qt::FontRole
    template <typename TModel> struct DataDispatcher<TModel, MovementColumns, Qt::FontRole> {
        static QVariant data(const TModel&, MovementColumns column) {
            if ((column == MovementColumns::DATE_VALUE) || (column == MovementColumns::AMOUNT) ||
                (column == MovementColumns::QUANTITY) || (column == MovementColumns::UNIT_VALUE)) {
                return QVariant{QFont{"Andale Mono"}};
            }
            return QVariant{};
        }

        template <typename TParent> static QVariant data(const TModel&, MovementColumns column, const TParent&) {
            if ((column == MovementColumns::DATE_VALUE) || (column == MovementColumns::AMOUNT) ||
                (column == MovementColumns::QUANTITY) || (column == MovementColumns::UNIT_VALUE)) {
                return QVariant{QFont{"Andale Mono"}};
            }
            return QVariant{};
        }
    };

    // Qt::BackgroundRole
    template <>
    template <>
    QVariant
    DataDispatcher<finances::investments::models::SnapshotNumerable, MovementColumns, Qt::BackgroundRole>::data<
        finances::accounts::models::Account>(const finances::investments::models::SnapshotNumerable&, MovementColumns,
                                             const finances::accounts::models::Account&);

    template <>
    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementColumns, Qt::BackgroundRole>::data<
        finances::accounts::models::Account>(const finances::accounts::models::Snapshot&, MovementColumns,
                                             const finances::accounts::models::Account&);

} // namespace utils::qt::models

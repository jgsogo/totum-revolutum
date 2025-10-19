#pragma once

#include <QFont>

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
    QUANTITY = 6,
    UNIT_VALUE = 7,
};

namespace utils::qt::models {

    // Qt::DisplayRole
    template <>
    template <>
    QVariant
    DataDispatcher<MovementModel, AccountMovementsColumns, Qt::DisplayRole>::data<finances::accounts::models::Account>(
        const MovementModel&, AccountMovementsColumns, const finances::accounts::models::Account&);

    template <>
    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, AccountMovementsColumns, Qt::DisplayRole>::data<
        finances::accounts::models::Account>(const finances::accounts::models::Snapshot&, AccountMovementsColumns,
                                             const finances::accounts::models::Account&);

    template <>
    template <>
    QVariant
    DataDispatcher<finances::investments::models::SnapshotNumerable, AccountMovementsColumns, Qt::DisplayRole>::data<
        finances::accounts::models::Account>(const finances::investments::models::SnapshotNumerable&,
                                             AccountMovementsColumns, const finances::accounts::models::Account&);

    // Qt::FontRole
    template <typename TModel> struct DataDispatcher<TModel, AccountMovementsColumns, Qt::FontRole> {
        static QVariant data(const TModel&, AccountMovementsColumns column) {
            if ((column == AccountMovementsColumns::DATE_VALUE) || (column == AccountMovementsColumns::AMOUNT) ||
                (column == AccountMovementsColumns::QUANTITY) || (column == AccountMovementsColumns::UNIT_VALUE)) {
                return QVariant{QFont{"Andale Mono"}};
            }
            return QVariant{};
        }

        template <typename TParent>
        static QVariant data(const TModel&, AccountMovementsColumns column, const TParent&) {
            if ((column == AccountMovementsColumns::DATE_VALUE) || (column == AccountMovementsColumns::AMOUNT) ||
                (column == AccountMovementsColumns::QUANTITY) || (column == AccountMovementsColumns::UNIT_VALUE)) {
                return QVariant{QFont{"Andale Mono"}};
            }
            return QVariant{};
        }
    };

    // Qt::BackgroundRole
    template <>
    template <>
    QVariant
    DataDispatcher<finances::investments::models::SnapshotNumerable, AccountMovementsColumns, Qt::BackgroundRole>::data<
        finances::accounts::models::Account>(const finances::investments::models::SnapshotNumerable&,
                                             AccountMovementsColumns, const finances::accounts::models::Account&);

    template <>
    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, AccountMovementsColumns, Qt::BackgroundRole>::data<
        finances::accounts::models::Account>(const finances::accounts::models::Snapshot&, AccountMovementsColumns,
                                             const finances::accounts::models::Account&);

} // namespace utils::qt::models

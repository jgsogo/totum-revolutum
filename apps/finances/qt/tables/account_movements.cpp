#include "account_movements.h"

#include <QBrush>
#include <QColor>
#include <QDate>
#include <QFont>

#include "libraries/finances/accounts/cpp/models/types/money.h"

namespace utils::qt::models {
    template <>
    QVariant DataDispatcher<MovementModel, AccountMovementsColumns, Qt::DisplayRole>::data(const MovementModel&,
                                                                                           AccountMovementsColumns) {
        return QVariant{"not yet"};
    }

    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, AccountMovementsColumns, Qt::DisplayRole>::data(
        const finances::accounts::models::Snapshot&, AccountMovementsColumns) {
        return QVariant{"not yet"};
    }

    template <>
    QVariant
    DataDispatcher<finances::investments::models::SnapshotNumerable, AccountMovementsColumns, Qt::DisplayRole>::data(
        const finances::investments::models::SnapshotNumerable&, AccountMovementsColumns) {
        return QVariant{"not yet"};
    }
} // namespace utils::qt::models

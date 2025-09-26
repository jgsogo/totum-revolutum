#include "account_related_snapshot.h"

#include <QColor>

void AccountRelatedSnapshotsModel::fetch_all_snapshots() { this->fetch_all(); }

QVariant AccountRelatedSnapshotsModel::data(const QModelIndex& index, int role) const {
    // Snapshot override some properties of their cells

    if (role == Qt::BackgroundRole) {
        QVariant result = QColor(255, 255, 40);
        return result;
    }

    return AccountRelatedModel<finances::accounts::models::Snapshot, SnapshotColumn>::data(index, role);
}

void AccountRelatedSnapshotsAsMovementsModel::fetch_all_snapshots() { this->fetch_all(); }

QVariant AccountRelatedSnapshotsAsMovementsModel::data(const QModelIndex& index, int role) const {
    // Snapshot override some properties of their cells

    if (role == Qt::BackgroundRole) {
        QVariant result = QColor(255, 255, 40);
        return result;
    }

    return AccountRelatedModel<finances::accounts::models::Snapshot, MovementColumn>::data(index, role);
}

template <>
QVariant AccountRelatedModel<finances::accounts::models::Snapshot, MovementColumn>::data_display_role(
    MovementColumn column, const finances::accounts::models::Snapshot& item) const {
    QVariant result = QVariant();
    switch (column) {
    case MovementColumn::ID:
        result = (uint64_t)item.id; // FIXME: implement the right conversion
        break;
    case MovementColumn::DATE_VALUE:
        result = QDate{int(item.date_value.year()), static_cast<int>(unsigned(item.date_value.month())),
                       static_cast<int>(unsigned(item.date_value.day()))}
                     .toString("yyyy-MM-dd");
        break;
    case MovementColumn::AMOUNT: {
        auto amount_money = finances::accounts::models::Money{item.amount, account.ccy};
        result = QString::fromStdString(static_cast<std::string>(amount_money));
    } break;
    case MovementColumn::TRANSACTION:
    case MovementColumn::MOVE_TYPE:
    case MovementColumn::DIRECTION:
        break;
    }
    return result;
}

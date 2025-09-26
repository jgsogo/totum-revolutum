#include "account_related_snapshot.h"

#include <QColor>

void AccountRelatedSnapshotsModel::fetch_all_snapshots() { this->fetch_all(); }
void AccountRelatedSnapshotsAsMovementsModel::fetch_all_snapshots() { this->fetch_all(); }

template <>
QVariant DataDispatcher<finances::accounts::models::Snapshot, SnapshotColumn, Qt::DisplayRole>::data(
    const finances::accounts::models::Account& account, const finances::accounts::models::Snapshot& item,
    SnapshotColumn column) {
    QVariant result = QVariant();

    switch (column) {
    case SnapshotColumn::ID:
        result = (uint64_t)item.id; // FIXME: implement the right conversion
        break;
    case SnapshotColumn::DATE_VALUE:
        result = QDate{int(item.date_value.year()), static_cast<int>(unsigned(item.date_value.month())),
                       static_cast<int>(unsigned(item.date_value.day()))}
                     .toString("yyyy-MM-dd");
        break;
    case SnapshotColumn::AMOUNT: {
        auto amount_money = finances::accounts::models::Money{item.amount, account.ccy};
        result = QString::fromStdString(static_cast<std::string>(amount_money));
    } break;
    }

    return result;
}

template <>
QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementColumn, Qt::DisplayRole>::data(
    const finances::accounts::models::Account& account, const finances::accounts::models::Snapshot& item,
    MovementColumn column) {
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

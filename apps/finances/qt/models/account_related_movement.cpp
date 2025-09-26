#include "account_related_movement.h"

#include <magic_enum/magic_enum.hpp>

void AccountRelatedMovementsModel::fetch_all_movements() { this->fetch_all(); }

template <>
QVariant DataDispatcher<finances::accounts::models::Movement, MovementColumn, Qt::DisplayRole>::data(
    const finances::accounts::models::Account& account, const finances::accounts::models::Movement& item,
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
        result = item.transaction.second.c_str();
        break;
    case MovementColumn::MOVE_TYPE:
        result = item.type.second.c_str();
        break;
    case MovementColumn::DIRECTION:
        result = QString::fromStdString(std::string(magic_enum::enum_name(item.direction)));
        break;
    }
    return result;
};

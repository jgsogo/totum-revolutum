#include "account_movements.h"

#include <QBrush>
#include <QColor>
#include <QDate>

#include "libraries/finances/accounts/cpp/models/types/money.h"

#include "apps/finances/qt/utils/utils.h"

namespace utils::qt::models {

    // namespace {
    //     QVariant display_role(const finances::accounts::models::Movement& movement, AccountMovementsColumns column,
    //                           const finances::accounts::models::Account& account) {
    //         QVariant result = QVariant();
    //         switch (column) {
    //         case AccountMovementsColumns::ID:
    //             result =
    //                 QString::fromStdString(std::format("{}", movement.id)); // FIXME: implement the right conversion
    //             break;
    //         case AccountMovementsColumns::DATE_VALUE:
    //             result = utils::date_to_qdate(movement.date_value).toString("yyyy-MM-dd");
    //             break;
    //         case AccountMovementsColumns::AMOUNT: {
    //             auto amount_money = finances::accounts::models::Money{movement.amount, account.ccy};
    //             result = QString::fromStdString(static_cast<std::string>(amount_money));
    //         } break;
    //         // Snapshot doesn't have these fields
    //         case AccountMovementsColumns::TRANSACTION:
    //         case AccountMovementsColumns::MOVE_TYPE:
    //         case AccountMovementsColumns::DIRECTION:
    //         case AccountMovementsColumns::QUANTITY:
    //         case AccountMovementsColumns::UNIT_VALUE:
    //             break;
    //         }
    //         return result;
    //     }
    // } // namespace

    template <>
    template <>
    QVariant
    DataDispatcher<MovementModel, AccountMovementsColumns, Qt::DisplayRole>::data<finances::accounts::models::Account>(
        const MovementModel& movement, AccountMovementsColumns column,
        const finances::accounts::models::Account& account) {
        QVariant result = QVariant();
        switch (column) {
        case AccountMovementsColumns::ID:
            result = QString::fromStdString(std::format("{}", movement.id)); // FIXME: implement the right conversion
            break;
        case AccountMovementsColumns::DATE_VALUE:
            result = utils::date_to_qdate(movement.date_value()).toString("yyyy-MM-dd");
            break;
        case AccountMovementsColumns::AMOUNT: {
            // auto amount_money = finances::accounts::models::Money{movement.movement.amount, account.ccy};
            // result = QString::fromStdString(static_cast<std::string>(amount_money));
        } break;
        // Snapshot doesn't have these fields
        case AccountMovementsColumns::TRANSACTION:
        case AccountMovementsColumns::MOVE_TYPE:
        case AccountMovementsColumns::DIRECTION:
        case AccountMovementsColumns::QUANTITY:
        case AccountMovementsColumns::UNIT_VALUE:
            break;
        }
        return result;
    }

    template <>
    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, AccountMovementsColumns, Qt::DisplayRole>::data<
        finances::accounts::models::Account>(const finances::accounts::models::Snapshot& snapshot,
                                             AccountMovementsColumns column,
                                             const finances::accounts::models::Account& account) {
        QVariant result = QVariant();

        switch (column) {
        case AccountMovementsColumns::ID:
            result = QString::fromStdString(std::format("{}", snapshot.id)); // FIXME: implement the right conversion
            break;
        case AccountMovementsColumns::DATE_VALUE:
            result = utils::date_to_qdate(snapshot.date_value).toString("yyyy-MM-dd");
            break;
        case AccountMovementsColumns::AMOUNT: {
            auto amount_money = finances::accounts::models::Money{snapshot.amount, account.ccy};
            result = QString::fromStdString(static_cast<std::string>(amount_money));
        } break;
        // Snapshot doesn't have these fields
        case AccountMovementsColumns::TRANSACTION:
        case AccountMovementsColumns::MOVE_TYPE:
        case AccountMovementsColumns::DIRECTION:
        case AccountMovementsColumns::QUANTITY:
        case AccountMovementsColumns::UNIT_VALUE:
            break;
        }

        return result;
    }

    template <>
    template <>
    QVariant
    DataDispatcher<finances::investments::models::SnapshotNumerable, AccountMovementsColumns, Qt::DisplayRole>::data<
        finances::accounts::models::Account>(const finances::investments::models::SnapshotNumerable& snapshot,
                                             AccountMovementsColumns column,
                                             const finances::accounts::models::Account& account) {
        QVariant result = QVariant();
        switch (column) {
        case AccountMovementsColumns::ID:
            result = QString::fromStdString(std::format("{}", snapshot.id)); // FIXME: implement the right conversion
            break;
        case AccountMovementsColumns::DATE_VALUE:
            result = utils::date_to_qdate(snapshot.snapshot.date_value).toString("yyyy-MM-dd");
            break;
        case AccountMovementsColumns::AMOUNT: {
            auto amount_money = finances::accounts::models::Money{snapshot.snapshot.amount, account.ccy};
            result = QString::fromStdString(static_cast<std::string>(amount_money));
        } break;
        case AccountMovementsColumns::QUANTITY:
            result = QString::fromStdString(static_cast<std::string>(snapshot.quantity));
            break;
        case AccountMovementsColumns::UNIT_VALUE: {
            auto unit_value_money = finances::accounts::models::Money{snapshot.unit_value, account.ccy};
            result = QString::fromStdString(static_cast<std::string>(unit_value_money));
        } break;
        // SnapshotNumerable doesn't have these fields
        case AccountMovementsColumns::TRANSACTION:
        case AccountMovementsColumns::MOVE_TYPE:
        case AccountMovementsColumns::DIRECTION:
            break;
        }
        return result;
    }

    template <>
    template <>
    QVariant
    DataDispatcher<finances::investments::models::SnapshotNumerable, AccountMovementsColumns, Qt::BackgroundRole>::data<
        finances::accounts::models::Account>(const finances::investments::models::SnapshotNumerable&,
                                             AccountMovementsColumns, const finances::accounts::models::Account&) {
        return QVariant{QColor(255, 255, 40)};
    }

    template <>
    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, AccountMovementsColumns, Qt::BackgroundRole>::data<
        finances::accounts::models::Account>(const finances::accounts::models::Snapshot&, AccountMovementsColumns,
                                             const finances::accounts::models::Account&) {
        return QVariant{QColor(255, 255, 40)};
    }
} // namespace utils::qt::models

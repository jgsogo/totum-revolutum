#include "movement_columns.h"

#include <QBrush>
#include <QColor>
#include <QDate>

#include "libraries/finances/accounts/cpp/models/types/money.h"

#include "apps/finances/qt/metatypes/types.h"
#include "apps/finances/qt/utils/utils.h"

using namespace finances::accounts::models;
using namespace finances::investments::models;

namespace utils::qt::models {

    template <>
    template <>
    QVariant DataDispatcher<MovementModel, MovementColumns, Qt::DisplayRole>::data<Account>(
        const MovementModel& movement, MovementColumns column, const Account& account) {
        QVariant result = QVariant();
        switch (column) {
        case MovementColumns::ID:
            result.setValue(movement.id);
            break;
        case MovementColumns::DATE_VALUE:
            result = utils::date_to_qdate(movement.as_movement().date_value).toString("yyyy-MM-dd");
            break;
        case MovementColumns::AMOUNT:
            result = QString::fromStdString(static_cast<std::string>(movement.as_movement().amount));
            break;
        case MovementColumns::MOVE_TYPE:
            result = QString::fromStdString(movement.movtype_breadcrumb);
            break;
        case MovementColumns::TRANSACTION:
            result = movement.as_movement().transaction.second.c_str();
            break;
        case MovementColumns::DIRECTION:
            result = QString::fromStdString(std::string(magic_enum::enum_name(movement.as_movement().direction)));
            break;
        case MovementColumns::QUANTITY:
            result = std::visit(
                [](const auto& arg) -> QVariant {
                    using T = std::decay_t<decltype(arg)>;
                    if constexpr (std::is_same_v<T, Movement>) {
                        return QVariant{};
                    } else if constexpr (std::is_same_v<T, MovementNumerable>) {
                        return QString::fromStdString(static_cast<std::string>(arg.quantity));
                    } else if constexpr (std::is_same_v<T, MovementDividend>) {
                        if (!arg.snapshot_data) {
                            SPDLOG_ERROR("Missing required data! Snapshot is mandatory for a MovementDividend!");
                            return QVariant{};
                        }
                        return QString::fromStdString(static_cast<std::string>(arg.snapshot_data.value().second));
                    } else {
                        static_assert(false, "non-exhaustive visitor!");
                    }
                },
                movement.movement);
            break;
        case MovementColumns::UNIT_VALUE:
            result = std::visit(
                [&account](const auto& arg) -> QVariant {
                    using T = std::decay_t<decltype(arg)>;
                    if constexpr (std::is_same_v<T, Movement>) {
                        return QVariant{};
                    } else if constexpr (std::is_same_v<T, MovementNumerable>) {
                        auto amount_money = Money{arg.unit_value, account.ccy};
                        return QString::fromStdString(static_cast<std::string>(amount_money));
                    } else if constexpr (std::is_same_v<T, MovementDividend>) {
                        auto amount_money = Money{arg.unit_value, account.ccy};
                        return QString::fromStdString(static_cast<std::string>(amount_money));
                    } else {
                        static_assert(false, "non-exhaustive visitor!");
                    }
                },
                movement.movement);
            break;
        case MovementColumns::ACCOUNT:
            result = movement.as_movement().account.second.c_str();
            break;
        case MovementColumns::ACCOUNT_ID:
            result.setValue(movement.as_movement().account.first);
            break;
        case MovementColumns::TRANSACTION_ID:
            result.setValue(movement.as_movement().transaction.first);
            break;
        }
        return result;
    }

    template <>
    template <>
    QVariant DataDispatcher<Snapshot, MovementColumns, Qt::DisplayRole>::data<Account>(const Snapshot& snapshot,
                                                                                       MovementColumns column,
                                                                                       const Account& account) {
        QVariant result = QVariant();

        switch (column) {
        case MovementColumns::ID:
            result.setValue(snapshot.id);
            break;
        case MovementColumns::DATE_VALUE:
            result = utils::date_to_qdate(snapshot.date_value).toString("yyyy-MM-dd");
            break;
        case MovementColumns::AMOUNT:
            result = QString::fromStdString(static_cast<std::string>(snapshot.amount));
            break;
        case MovementColumns::ACCOUNT:
            result = snapshot.account.second.c_str();
            break;
        case MovementColumns::ACCOUNT_ID:
            result.setValue(snapshot.account.first);
            break;
        // Snapshot doesn't have these fields
        case MovementColumns::TRANSACTION:
        case MovementColumns::MOVE_TYPE:
        case MovementColumns::DIRECTION:
        case MovementColumns::QUANTITY:
        case MovementColumns::UNIT_VALUE:
        case MovementColumns::TRANSACTION_ID:
            break;
        }

        return result;
    }

    template <>
    template <>
    QVariant DataDispatcher<SnapshotNumerable, MovementColumns, Qt::DisplayRole>::data<Account>(
        const SnapshotNumerable& snapshot, MovementColumns column, const Account& account) {
        QVariant result = QVariant();
        switch (column) {
        case MovementColumns::ID:
            result = QString::fromStdString(std::format("{}", snapshot.id)); // FIXME: implement the right conversion
            break;
        case MovementColumns::DATE_VALUE:
            result = utils::date_to_qdate(snapshot.snapshot.date_value).toString("yyyy-MM-dd");
            break;
        case MovementColumns::AMOUNT:
            result = QString::fromStdString(static_cast<std::string>(snapshot.snapshot.amount));
            break;
        case MovementColumns::QUANTITY:
            result = QString::fromStdString(static_cast<std::string>(snapshot.quantity));
            break;
        case MovementColumns::UNIT_VALUE: {
            auto unit_value_money = Money{snapshot.unit_value, account.ccy};
            result = QString::fromStdString(static_cast<std::string>(unit_value_money));
        } break;
        case MovementColumns::ACCOUNT:
            result = snapshot.snapshot.account.second.c_str();
            break;
        case MovementColumns::ACCOUNT_ID:
            result.setValue(snapshot.snapshot.account.first);
            break;
        // SnapshotNumerable doesn't have these fields
        case MovementColumns::TRANSACTION:
        case MovementColumns::MOVE_TYPE:
        case MovementColumns::DIRECTION:
        case MovementColumns::TRANSACTION_ID:
            break;
        }
        return result;
    }

    template <>
    template <>
    QVariant DataDispatcher<SnapshotNumerable, MovementColumns, Qt::BackgroundRole>::data<Account>(
        const SnapshotNumerable&, MovementColumns, const Account&) {
        return QVariant{QColor(255, 255, 40)};
    }

    template <>
    template <>
    QVariant DataDispatcher<Snapshot, MovementColumns, Qt::BackgroundRole>::data<Account>(const Snapshot&,
                                                                                          MovementColumns,
                                                                                          const Account&) {
        return QVariant{QColor(255, 255, 40)};
    }
} // namespace utils::qt::models

#include "movement_columns.h"

#include <QBrush>
#include <QColor>
#include <QDate>

#include "libraries/finances/accounts/cpp/models/types/money.h"

#include "apps/finances/qt/metatypes/types.h"
#include "apps/finances/qt/utils/utils.h"

namespace utils::qt::models {

    template <>
    template <>
    QVariant DataDispatcher<MovementModel, MovementColumns, Qt::DisplayRole>::data<finances::accounts::models::Account>(
        const MovementModel& movement, MovementColumns column, const finances::accounts::models::Account& account) {
        QVariant result = QVariant();
        switch (column) {
        case MovementColumns::ID:
            result.setValue(movement.id);
            break;
        case MovementColumns::DATE_VALUE:
            result = utils::date_to_qdate(movement.as_movement().date_value).toString("yyyy-MM-dd");
            break;
        case MovementColumns::AMOUNT: {
            auto amount_money = finances::accounts::models::Money{movement.as_movement().amount, account.ccy};
            result = QString::fromStdString(static_cast<std::string>(amount_money));
        } break;
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
                    if constexpr (std::is_same_v<T, finances::accounts::models::Movement>) {
                        return QVariant{};
                    } else if constexpr (std::is_same_v<T, finances::investments::models::MovementNumerable>) {
                        return QString::fromStdString(static_cast<std::string>(arg.quantity));
                    } else if constexpr (std::is_same_v<T, finances::investments::models::MovementDividend>) {
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
                    if constexpr (std::is_same_v<T, finances::accounts::models::Movement>) {
                        return QVariant{};
                    } else if constexpr (std::is_same_v<T, finances::investments::models::MovementNumerable>) {
                        auto amount_money = finances::accounts::models::Money{arg.unit_value, account.ccy};
                        return QString::fromStdString(static_cast<std::string>(amount_money));
                    } else if constexpr (std::is_same_v<T, finances::investments::models::MovementDividend>) {
                        auto amount_money = finances::accounts::models::Money{arg.unit_value, account.ccy};
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
            // result = QString::fromStdString("%1 -").arg(std::format("{}", movement.as_movement().account.first));
            break;
        case MovementColumns::TRANSACTION_ID:
            result.setValue(movement.as_movement().transaction.first);
            break;
        }
        return result;
    }

    template <>
    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementColumns, Qt::DisplayRole>::data<
        finances::accounts::models::Account>(const finances::accounts::models::Snapshot& snapshot,
                                             MovementColumns column,
                                             const finances::accounts::models::Account& account) {
        QVariant result = QVariant();

        switch (column) {
        case MovementColumns::ID:
            result = QString::fromStdString(std::format("{}", snapshot.id)); // FIXME: implement the right conversion
            break;
        case MovementColumns::DATE_VALUE:
            result = utils::date_to_qdate(snapshot.date_value).toString("yyyy-MM-dd");
            break;
        case MovementColumns::AMOUNT: {
            auto amount_money = finances::accounts::models::Money{snapshot.amount, account.ccy};
            result = QString::fromStdString(static_cast<std::string>(amount_money));
        } break;
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
    QVariant DataDispatcher<finances::investments::models::SnapshotNumerable, MovementColumns, Qt::DisplayRole>::data<
        finances::accounts::models::Account>(const finances::investments::models::SnapshotNumerable& snapshot,
                                             MovementColumns column,
                                             const finances::accounts::models::Account& account) {
        QVariant result = QVariant();
        switch (column) {
        case MovementColumns::ID:
            result = QString::fromStdString(std::format("{}", snapshot.id)); // FIXME: implement the right conversion
            break;
        case MovementColumns::DATE_VALUE:
            result = utils::date_to_qdate(snapshot.snapshot.date_value).toString("yyyy-MM-dd");
            break;
        case MovementColumns::AMOUNT: {
            auto amount_money = finances::accounts::models::Money{snapshot.snapshot.amount, account.ccy};
            result = QString::fromStdString(static_cast<std::string>(amount_money));
        } break;
        case MovementColumns::QUANTITY:
            result = QString::fromStdString(static_cast<std::string>(snapshot.quantity));
            break;
        case MovementColumns::UNIT_VALUE: {
            auto unit_value_money = finances::accounts::models::Money{snapshot.unit_value, account.ccy};
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
    QVariant
    DataDispatcher<finances::investments::models::SnapshotNumerable, MovementColumns, Qt::BackgroundRole>::data<
        finances::accounts::models::Account>(const finances::investments::models::SnapshotNumerable&, MovementColumns,
                                             const finances::accounts::models::Account&) {
        return QVariant{QColor(255, 255, 40)};
    }

    template <>
    template <>
    QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementColumns, Qt::BackgroundRole>::data<
        finances::accounts::models::Account>(const finances::accounts::models::Snapshot&, MovementColumns,
                                             const finances::accounts::models::Account&) {
        return QVariant{QColor(255, 255, 40)};
    }
} // namespace utils::qt::models

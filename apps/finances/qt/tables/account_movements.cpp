#include "account_movements.h"

#include <QBrush>
#include <QColor>
#include <QDate>

#include "libraries/finances/accounts/cpp/models/types/money.h"

#include "apps/finances/qt/utils/utils.h"

namespace utils::qt::models {

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
            result = utils::date_to_qdate(movement.as_movement().date_value).toString("yyyy-MM-dd");
            break;
        case AccountMovementsColumns::AMOUNT: {
            auto amount_money = finances::accounts::models::Money{movement.as_movement().amount, account.ccy};
            result = QString::fromStdString(static_cast<std::string>(amount_money));
        } break;
        case AccountMovementsColumns::MOVE_TYPE:
            result = QString::fromStdString(movement.movtype_breadcrumb);
            break;
        case AccountMovementsColumns::TRANSACTION:
            result = movement.as_movement().transaction.second.c_str();
            break;
        case AccountMovementsColumns::DIRECTION:
            result = QString::fromStdString(std::string(magic_enum::enum_name(movement.as_movement().direction)));
            break;
        case AccountMovementsColumns::QUANTITY:
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
        case AccountMovementsColumns::UNIT_VALUE:
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

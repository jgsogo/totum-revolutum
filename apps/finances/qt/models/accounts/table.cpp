#include "table.h"

#include <QBrush>
#include <QColor>
#include <QDate>
#include <QFont>

#include "libraries/finances/accounts/cpp/models/types/money.h"

AccountsTable::AccountsTable(utils::libpqxx::ConnectionPool& pool, QObject* parent)
    : utils::qt::models::TableModel<AccountModel, AccountColumns>{pool, parent} {}

void AccountsTable::_refresh_all() {
    SPDLOG_DEBUG("AccountsTable::_refresh_all");
    utils::qt::models::TableModel<AccountModel, AccountColumns>::_refresh_all();

    // Populate breadcrumb
    utils::db::AccountTypeManager acctype_manager{pool};
    for (auto& item : items) {
        auto breadcrumb = acctype_manager.breadcrumb(item.account.type.first);
        std::string breadcrumb_str;
        if (!breadcrumb) {
            SPDLOG_WARN("Error retrieving breadcrumb for account {}: {}", item.id, breadcrumb.error());
            // TODO: Communicate error to user
            breadcrumb_str = item.account.type.second;
        } else {
            for (auto it : breadcrumb.value()) {
                breadcrumb_str.append(it);
                breadcrumb_str.append(" > ");
            }
            breadcrumb_str.append(item.account.type.second);
        }
        item.account_type_breadcrumb = breadcrumb_str;
    }
};

void AccountsTable::_refresh_one(const AccountModel::Id& id, int row) {
    SPDLOG_DEBUG("AccountsTable::_refresh_one(id={}, row={})", id, row);

    // For a given row, the only thing that can change is the last_snapshot
    utils::db::SnapshotManager manager{pool};
    auto last_snapshot = manager.get_last_snapshot(id);
    if (!last_snapshot) {
        SPDLOG_ERROR("Error retrieving last snapshot for snapshot {}: {}", id, last_snapshot.error());
        // TODO: Communicate error to user
        return;
    }

    this->items.at(row).last_snapshot = std::move(last_snapshot.value());

    // Emit a signal to notify that the snapshot column has been modified
    QVector<int> roles = {Qt::DisplayRole};
    QModelIndex topLeft = this->createIndex(row, magic_enum::enum_integer(AccountColumns::SNAPSHOT));
    emit dataChanged(topLeft, topLeft, roles);
}
namespace utils::qt::models {

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::DisplayRole>::data(
        const TableModel<AccountModel, AccountColumns>&, const AccountModel& item, AccountColumns column) {
        QVariant result;

        const finances::accounts::models::Account& account = item.account;

        switch (column) {
        case AccountColumns::ID:
            result = (uint64_t)item.id; // FIXME: implement the right conversion
            break;
        case AccountColumns::CUSTODIAN:
            result = account.custodian.second.c_str();
            break;
        case AccountColumns::NAME: {
            // FIXME: use account.isClosed()
            if (account.close) {
                auto close_date = QDate{int(account.close->year()), static_cast<int>(unsigned(account.close->month())),
                                        static_cast<int>(unsigned(account.close->day()))};
                if (close_date < QDate::currentDate()) {
                    result = QString("%1 🔒").arg(account.name);
                } else {
                    result = account.name.c_str();
                }
            } else {
                result = account.name.c_str();
            }
        } break;
        case AccountColumns::IDENTIFIER:
            result = account.identifier.value_or("").c_str();
            break;
        case AccountColumns::SNAPSHOT: {
            const auto& snapshot = item.last_snapshot;
            if (snapshot) {
                // FIXME: Implement some convenient functions in Snapshot class
                auto snapshot_money = finances::accounts::models::Money{snapshot.value().amount, account.ccy};
                auto date_value = QDate{int(snapshot.value().date_value.year()),
                                        static_cast<int>(unsigned(snapshot.value().date_value.month())),
                                        static_cast<int>(unsigned(snapshot.value().date_value.day()))};
                if (date_value.daysTo(QDate::currentDate()) > 21) {
                    result = QString("🗓️ %1").arg(static_cast<std::string>(snapshot_money));
                } else {
                    result = QString::fromStdString(static_cast<std::string>(snapshot_money));
                }
            } else {
                result = QString("❗");
            }
        } break;
        case AccountColumns::TYPE: {
            result = QString::fromStdString(item.account_type_breadcrumb);
            // auto found = _account_type_breadcrumb.find(account.type.first);
            // if (found != _account_type_breadcrumb.end()) {
            //     result = found->second;
            // }
        } break;
        case AccountColumns::OPEN:
            result = QDate{int(account.open.year()), static_cast<int>(unsigned(account.open.month())),
                           static_cast<int>(unsigned(account.open.day()))}
                         .toString("yyyy-MM-dd");
            break;
        case AccountColumns::CLOSE:
            if (account.close) {
                result = QDate{int(account.close->year()), static_cast<int>(unsigned(account.close->month())),
                               static_cast<int>(unsigned(account.close->day()))}
                             .toString("yyyy-MM-dd");
            }
            break;
        case AccountColumns::HOLDERS: {
            const auto& account_holders = item.holders;

            // TODO: Implement this implode-transform as a util
            const char* const delim = ", ";
            std::ostringstream imploded;
            std::transform(account_holders.begin(), account_holders.end(),
                           std::ostream_iterator<std::string>(imploded, delim),
                           [](const auto& acc_holder) { return acc_holder.first.name; });
            result = QString::fromStdString(imploded.str());
        } break;
        }
        return result;
    }

    template <>
    QVariant
    DataDispatcher<AccountModel, AccountColumns, Qt::FontRole>::data(const TableModel<AccountModel, AccountColumns>&,
                                                                     const AccountModel&, AccountColumns column) {
        QVariant result;
        if ((column == AccountColumns::SNAPSHOT) || (column == AccountColumns::OPEN) ||
            (column == AccountColumns::CLOSE) || (column == AccountColumns::IDENTIFIER)) {
            result = QFont{"Andale Mono"};
        }
        return result;
    }

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::ForegroundRole>::data(
        const TableModel<AccountModel, AccountColumns>&, const AccountModel& item, AccountColumns) {
        QVariant result;
        const auto& account = item.account;
        if (account.close) {
            auto close_date = QDate{int(account.close->year()), static_cast<int>(unsigned(account.close->month())),
                                    static_cast<int>(unsigned(account.close->day()))};
            if (close_date < QDate::currentDate()) {
                result = QBrush{QColor{Qt::darkGray}};
            }
        }
        return result;
    }

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::TextAlignmentRole>::data(
        const TableModel<AccountModel, AccountColumns>&, const AccountModel&, AccountColumns column) {
        if ((column == AccountColumns::CUSTODIAN) || (column == AccountColumns::NAME) ||
            (column == AccountColumns::TYPE)) {
            return Qt::AlignLeft;
        } else {
            return Qt::AlignRight;
        }
    }

} // namespace utils::qt::models

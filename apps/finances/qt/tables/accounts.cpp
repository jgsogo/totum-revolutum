#include "accounts.h"

#include <QBrush>
#include <QColor>
#include <QDate>
#include <QFont>

#include "libraries/finances/accounts/cpp/models/types/money.h"

#include "apps/finances/qt/metatypes/types.h"
#include "apps/finances/qt/utils/utils.h"

using namespace finances::accounts::models;

namespace utils::qt::models {

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::DisplayRole>::data(const AccountModel& item,
                                                                                 AccountColumns column) {
        QVariant result;

        const Account& account = item.account;

        switch (column) {
        case AccountColumns::ID:
            result.setValue(item.id);
            break;
        case AccountColumns::CUSTODIAN:
            result = account.custodian.second.c_str();
            break;
        case AccountColumns::NAME: {
            // FIXME: use account.isClosed()
            if (account.close) {
                auto close_date = utils::date_to_qdate(account.close.value());
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
                auto date_value = utils::date_to_qdate(snapshot.value().date_value);
                if (date_value.daysTo(QDate::currentDate()) > 21) { // FIXME: Hardcoded number
                    result = QString("🗓️ %1").arg(static_cast<std::string>(snapshot.value().amount));
                } else {
                    result = QString::fromStdString(static_cast<std::string>(snapshot.value().amount));
                }
            } else {
                result = QString("❗");
            }
        } break;
        case AccountColumns::TYPE:
            result = QString::fromStdString(item.account_type_breadcrumb);
            break;
        case AccountColumns::OPEN:
            result = utils::date_to_qdate(account.open).toString("yyyy-MM-dd");
            break;
        case AccountColumns::CLOSE:
            if (account.close) {
                result = utils::date_to_qdate(account.close.value()).toString("yyyy-MM-dd");
            }
            break;
        case AccountColumns::HOLDERS: {
            const auto& account_holders = item.holders;

            // TODO: Implement this implode-transform as a util
            const char* const delim = ", ";
            std::ostringstream imploded;
            std::transform(account_holders.begin(), account_holders.end(),
                           std::ostream_iterator<std::string>(imploded, delim),
                           [](const auto& acc_holder) { return acc_holder.name; });
            result = QString::fromStdString(imploded.str());
        } break;
        }
        return result;
    }

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::FontRole>::data(const AccountModel&,
                                                                              AccountColumns column) {
        QVariant result;
        if ((column == AccountColumns::SNAPSHOT) || (column == AccountColumns::OPEN) ||
            (column == AccountColumns::CLOSE) || (column == AccountColumns::IDENTIFIER)) {
            result = QFont{"Andale Mono"};
        }
        return result;
    }

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::ForegroundRole>::data(const AccountModel& item,
                                                                                    AccountColumns) {
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
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::TextAlignmentRole>::data(const AccountModel&,
                                                                                       AccountColumns column) {
        if ((column == AccountColumns::CUSTODIAN) || (column == AccountColumns::NAME) ||
            (column == AccountColumns::TYPE)) {
            return Qt::AlignLeft;
        } else {
            return Qt::AlignRight;
        }
    }

} // namespace utils::qt::models

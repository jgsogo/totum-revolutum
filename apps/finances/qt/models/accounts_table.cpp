#include "accounts_table.h"

#include <QBrush>
#include <QColor>
#include <QDate>
#include <QFont>
#include <QTimer>
#include <magic_enum/magic_enum.hpp>

#include "libraries/finances/accounts/cpp/models/types/money.h"
#include "libraries/utils/cpp/enumerate.hpp"

AccountTableModel::AccountTableModel(utils::libpqxx::ConnectionPool& pool, QObject* parent)
    : QAbstractTableModel(parent), pool{pool} {
    QTimer::singleShot(0, this, SLOT(fetch_all()));
}

int AccountTableModel::rowCount(const QModelIndex&) const { return accounts.size(); }
int AccountTableModel::columnCount(const QModelIndex&) const { return magic_enum::enum_count<Column>(); }

QVariant AccountTableModel::data(const QModelIndex& index, int role) const {
    QVariant result = QVariant();

    int row = index.row();
    int column_idx = index.column();

    if (!index.isValid() || row >= rowCount() || column_idx >= columnCount()) {
        return result;
    }

    Column column = magic_enum::enum_value<Column>(column_idx);
    const auto& account = accounts.at(row);

    switch (role) {
    case Qt::DisplayRole: {
        switch (column) {
        case Column::ID:
            result = (uint64_t)account.id; // FIXME: implement the right conversion
            break;
        case Column::CUSTODIAN:
            result = account.custodian.second.c_str();
            break;
        case Column::NAME: {
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
        case Column::IDENTIFIER:
            result = account.identifier.value_or("").c_str();
            break;
        case Column::SNAPSHOT: {
            const auto& snapshot = snapshots.at(row);
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
                result = QString(tr("❗"));
            }
        } break;
        case Column::TYPE: {
            auto found = _account_type_breadcrumb.find(account.type.first);
            if (found != _account_type_breadcrumb.end()) {
                result = found->second;
            }
        } break;
        case Column::OPEN:
            result = QDate{int(account.open.year()), static_cast<int>(unsigned(account.open.month())),
                           static_cast<int>(unsigned(account.open.day()))}
                         .toString("yyyy-MM-dd");
            break;
        case Column::CLOSE:
            if (account.close) {
                result = QDate{int(account.close->year()), static_cast<int>(unsigned(account.close->month())),
                               static_cast<int>(unsigned(account.close->day()))}
                             .toString("yyyy-MM-dd");
            }
            break;
        case Column::HOLDERS: {
            const auto& account_holders = holders.at(row);

            // TODO: Implement this implode-transform as a util
            const char* const delim = ", ";
            std::ostringstream imploded;
            std::transform(account_holders.begin(), account_holders.end(),
                           std::ostream_iterator<std::string>(imploded, delim),
                           [](const auto& acc_holder) { return acc_holder.first.name; });
            result = QString::fromStdString(imploded.str());
        } break;
        }
    } break;
    case Qt::FontRole:
        if ((column == Column::SNAPSHOT) || (column == Column::OPEN) || (column == Column::CLOSE)) {
            result = QFont{"Andale Mono"};
        }
        break;
    case Qt::ForegroundRole: {
        // FIXME: use account.isClosed()
        if (account.close) {
            auto close_date = QDate{int(account.close->year()), static_cast<int>(unsigned(account.close->month())),
                                    static_cast<int>(unsigned(account.close->day()))};
            if (close_date < QDate::currentDate()) {
                result = QBrush{QColor{Qt::darkGray}};
            }
        }
    } break;
    case Qt::TextAlignmentRole:
        if ((column == Column::CUSTODIAN) || (column == Column::NAME) || (column == Column::TYPE)) {
            result = Qt::AlignLeft;
        } else {
            result = Qt::AlignRight;
        }
        break;
    default:
        break;
    }

    return result;
}

QVariant AccountTableModel::headerData(int section, Qt::Orientation orientation, int role) const {
    /*
    Returns the data for the given role and section in the header with the specified orientation.

    For horizontal headers, the section number corresponds to the column number. Similarly,
    for vertical headers, the section number corresponds to the row number.
    */

    QVariant result = QVariant();

    if (role == Qt::DisplayRole && orientation == Qt::Horizontal) { // H
        Column column = magic_enum::enum_value<Column>(section);
        result = QString::fromStdString(std::string(magic_enum::enum_name(column)));
    } else if (role == Qt::DisplayRole && orientation == Qt::Vertical) { // V
        return QString("%1").arg(accounts[section].id);
    } else {
        // other stuff
    }
    return result;
}

void AccountTableModel::fetch_all() {
    SPDLOG_DEBUG("AccountTableModel::fetch_all");

    SPDLOG_TRACE(" - fetch all accounts");
    finances::accounts::models::AccountManager manager{pool};
    auto all_accounts = manager.all();
    if (!all_accounts) {
        SPDLOG_ERROR("Error refreshing accounts");
        // TODO: Communicate error to user
        return;
    }

    // Fetch the account_type breadcrumbs
    SPDLOG_TRACE(" - fetch all account_type breadcrumbs");
    // FIXME: The breadcrumbs could/should be created only once, maybe at the root of the application
    std::map<finances::accounts::models::Id, QString> account_type_breadcrumb;
    finances::accounts::models::AccountType::Manager account_type_manager{pool};
    auto all_account_type = account_type_manager.all();
    if (all_account_type) {
        for (const auto& acc_type : all_account_type.value()) {
            // FIXME: We are doing this also for the movement types
            QString q_breadcrumb;
            auto breadcrumb = account_type_manager.breadcrumb(acc_type.id);
            if (breadcrumb) {
                for (auto it : breadcrumb.value()) {
                    q_breadcrumb.append(it.c_str());
                    q_breadcrumb.append(" > ");
                }
                q_breadcrumb.append(acc_type.name.c_str());
                account_type_breadcrumb.insert(std::make_pair(acc_type.id, q_breadcrumb));
            }
        }
    } else {
        SPDLOG_ERROR("Failed to get all the AccountType instances");
        // TODO: Communicate error to user
    }

    // Create empty snapshots vector
    std::vector<std::optional<finances::accounts::models::Snapshot>> all_snapshots(all_accounts->size(), std::nullopt);

    // Create empty account_holders vector
    std::vector<std::vector<
        std::pair<finances::accounts::models::AccountHolder, finances::accounts::models::AccountHolderRole>>>
        initial_holders(
            all_accounts->size(),
            std::vector<
                std::pair<finances::accounts::models::AccountHolder, finances::accounts::models::AccountHolderRole>>{});

    this->beginResetModel();
    this->accounts = std::move(all_accounts.value());
    this->snapshots = std::move(all_snapshots);
    this->holders = std::move(initial_holders);
    this->_account_type_breadcrumb = std::move(account_type_breadcrumb);
    this->endResetModel();

    // We have updated all the accounts, so let's fetch all the snapshots together.
    QTimer::singleShot(0, this, SLOT(fetch_snapshots()));
    QTimer::singleShot(0, this, SLOT(fetch_account_holders()));
}

void AccountTableModel::fetch_snapshots() {
    SPDLOG_DEBUG("AccountTableModel::fetch_snapshots");
    finances::accounts::models::SnapshotManager manager{pool};

    std::vector<finances::accounts::models::Id> account_ids;
    account_ids.resize(accounts.size());
    std::transform(accounts.begin(), accounts.end(), account_ids.begin(),
                   [](const auto& account) { return account.id; });
    auto last_snapshots = manager.get_last_snapshots(account_ids);

    if (!last_snapshots) {
        SPDLOG_ERROR("Error refreshing snapshots");
        // TODO: Communicate error to user
        return;
    }

    this->snapshots = std::move(last_snapshots.value());

    QVector<int> roles = {Qt::DisplayRole};
    QModelIndex topLeft = this->createIndex(0, magic_enum::enum_integer(Column::SNAPSHOT));
    QModelIndex bottomRight = this->createIndex(rowCount(), magic_enum::enum_integer(Column::SNAPSHOT));
    emit dataChanged(topLeft, bottomRight, roles);
}

void AccountTableModel::fetch_account_holders() {
    SPDLOG_DEBUG("AccountTableModel::fetch_account_holders");
    finances::accounts::models::AccountHolderManager manager{pool};

    std::vector<std::vector<
        std::pair<finances::accounts::models::AccountHolder, finances::accounts::models::AccountHolderRole>>>
        account_holders;
    account_holders.resize(accounts.size());
    for (const auto& [i, account] : utils::enumerate(accounts)) {
        auto holders_expected = manager.all(account.id);
        if (!holders_expected) {
            SPDLOG_WARN("Error retrieving AccountHolders for account {}", account.id);
            continue;
        }
        account_holders[i] = std::move(holders_expected.value());
    }

    this->holders = std::move(account_holders);

    QVector<int> roles = {Qt::DisplayRole};
    QModelIndex topLeft = this->createIndex(0, magic_enum::enum_integer(Column::HOLDERS));
    QModelIndex bottomRight = this->createIndex(rowCount(), magic_enum::enum_integer(Column::HOLDERS));
    emit dataChanged(topLeft, bottomRight, roles);
}

void AccountTableModel::fetch_snapshot(finances::accounts::models::Id account_id) {
    SPDLOG_DEBUG("AccountTableModel::fetch_snapshot(account_id={})", account_id);

    // Find the row for the fetched snapshot
    auto it = std::find_if(this->accounts.begin(), this->accounts.end(),
                           [&account_id](const auto& account) { return account.id == account_id; });
    if (it == this->accounts.end()) {
        SPDLOG_ERROR("Account {} is not in the model", account_id);
        // TODO: Communicate error to user
        return;
    }

    finances::accounts::models::SnapshotManager manager{pool};
    auto last_snapshot = manager.get_last_snapshot(account_id);
    if (!last_snapshot) {
        SPDLOG_ERROR("Error fetching snapshot for account: {}", account_id);
        // TODO: Communicate error to user
        return;
    }

    // Update the corresponding row
    auto row = std::distance(this->accounts.begin(), it);
    this->snapshots.at(row).swap(last_snapshot.value());

    QVector<int> roles = {Qt::DisplayRole};
    QModelIndex topLeft = this->createIndex(row, magic_enum::enum_integer(Column::SNAPSHOT));
    emit dataChanged(topLeft, topLeft, roles);
}

const finances::accounts::models::Account&
AccountTableModel::get_account(finances::accounts::models::Id account_id) const {
    SPDLOG_TRACE("AccountTableModel::get_account(account_id={})", account_id);
    auto found =
        std::find_if(accounts.begin(), accounts.end(), [&account_id](const auto& acc) { return acc.id == account_id; });
    if (found == accounts.end()) {
        SPDLOG_ERROR(" - Unexpected: Account not found!");
        throw std::runtime_error("Details requested for account that doesn't exist!");
    }

    return *found;
}

const finances::accounts::models::Account& AccountTableModel::get_account(int row) const {
    SPDLOG_TRACE("AccountTableModel::get_account(row={})", row);
    return accounts.at(row);
}

const std::vector<std::pair<finances::accounts::models::AccountHolder, finances::accounts::models::AccountHolderRole>>&
AccountTableModel::get_holders(int row) const {
    SPDLOG_TRACE("AccountTableModel::get_holders(row={})", row);
    return holders.at(row);
}

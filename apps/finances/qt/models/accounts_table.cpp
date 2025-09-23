#include "accounts_table.h"

#include <QDate>
#include <QFont>
#include <QTimer>
#include <magic_enum/magic_enum.hpp>

#include "libraries/finances/accounts/cpp/models/types/money.h"

namespace {
    enum class Column {
        CUSTODIAN = 0,
        NAME = 1,
        IDENTIFIER = 2,
        SNAPSHOT = 3,
        TYPE = 4,
        OPEN = 5,
        CLOSE = 6,
    };
}

AccountTableModel::AccountTableModel(utils::libpqxx::ConnectionPool& pool, QObject* parent)
    : QAbstractTableModel(parent), pool{pool} {}

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

    switch (role) {
    case Qt::DisplayRole: {
        const auto& account = accounts.at(row);
        switch (column) {
        case Column::CUSTODIAN:
            result = account.custodian.second.c_str();
            break;
        case Column::NAME:
            result = account.name.c_str();
            break;
        case Column::IDENTIFIER:
            result = account.identifier.value_or("").c_str();
            break;
        case Column::SNAPSHOT: {
            const auto& snapshot = snapshots.at(row);
            if (snapshot) {
                auto snapshot_money = finances::accounts::models::Money{snapshot.value().amount, account.ccy};
                result = QString::fromStdString(static_cast<std::string>(snapshot_money));
            }
        } break;
        case Column::TYPE:
            result = account.type.second.c_str();
            break;
        case Column::OPEN:
            result = QDate{int(account.open.year()), static_cast<int>(unsigned(account.open.month())),
                           static_cast<int>(unsigned(account.open.day()))}
                         .toString("yyyy-MM-dd");
            break;
        case Column::CLOSE:
            result = QDate{int(account.close->year()), static_cast<int>(unsigned(account.close->month())),
                           static_cast<int>(unsigned(account.close->day()))}
                         .toString("yyyy-MM-dd");
        }
    } break;
    case Qt::FontRole:
        if (column == Column::SNAPSHOT) {
            result = QFont{"Andale Mono"};
        }
        break;
    //
    // case Qt::ForegroundRole:
    //     if (1 == column) {
    //         result = QColor(Qt::red);
    //     }
    //     break;
    // //
    // case Qt::BackgroundRole:
    //     if (1 == column) {
    //         result = QColor(255, 255, 40);
    //     }
    //     break;
    // //
    case Qt::TextAlignmentRole:
        result = Qt::AlignRight;
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
    finances::accounts::models::AccountManager manager{pool};
    auto all_accounts = manager.all();
    if (!all_accounts) {
        SPDLOG_ERROR("Error refreshing accounts");
        // TODO: Communicate error to user
        return;
    }

    std::vector<std::optional<finances::accounts::models::Snapshot>> all_snapshots(all_accounts->size(), std::nullopt);

    this->beginResetModel();
    this->accounts = std::move(all_accounts.value());
    this->snapshots = std::move(all_snapshots);
    this->endResetModel();

    // We have updated all the accounts, so let's fetch all the snapshots together.
    QTimer::singleShot(0, this, SLOT(fetch_snapshots()));
}

void AccountTableModel::fetch_snapshots() {
    SPDLOG_DEBUG("AccountTableModel::fetch_snapshots");
    finances::accounts::models::AccountManager manager{pool};

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

    finances::accounts::models::AccountManager manager{pool};
    auto last_snapshot = manager.get_last_snapshot(account_id);
    if (!last_snapshot) {
        SPDLOG_ERROR("Error fetching snapshot for account: {}", account_id);
        // TODO: Communicate error to user
        return;
    }

    // Update the corresponding row
    auto row = std::distance(this->accounts.begin(), it);
    this->snapshots.at(row) = std::move(last_snapshot.value());

    QVector<int> roles = {Qt::DisplayRole};
    QModelIndex topLeft = this->createIndex(row, magic_enum::enum_integer(Column::SNAPSHOT));
    emit dataChanged(topLeft, topLeft, roles);
}

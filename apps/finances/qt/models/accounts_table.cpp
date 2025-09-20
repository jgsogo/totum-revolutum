#include "accounts_table.h"

AccountTableModel::AccountTableModel(std::vector<finances::accounts::models::Account>&& accounts, QObject* parent)
    : QAbstractTableModel(parent), accounts{std::move(accounts)} {}

AccountTableModel* AccountTableModel::create_with_all(utils::db::ConnectionPool& pool, QObject* parent) {
    finances::accounts::models::AccountManager manager{pool};
    auto all_accounts = manager.all();
    return new AccountTableModel(std::move(all_accounts.value()), parent);
}

int AccountTableModel::rowCount(const QModelIndex& parent) const { return accounts.size(); }
int AccountTableModel::columnCount(const QModelIndex& parent) const { return 3; }

QVariant AccountTableModel::data(const QModelIndex& index, int role) const {
    QVariant result = QVariant();

    int row = index.row();
    int column = index.column();

    if (!index.isValid() || row >= rowCount() || column >= columnCount()) {
        return result;
    }

    switch (role) {
    case Qt::DisplayRole: {
        const auto& account = accounts.at(row);
        if (column == 0) {
            result = account.name.c_str();
        } else if (column == 1) {
            result = account.identifier.value_or("").c_str();
        } else if (column == 2) {
            result = QString::fromStdString(static_cast<std::string>(account.ccy));
        }
    }
    // result = QString("row-%1, col-%2").arg(row).arg(column);
    break;
    // case Qt::FontRole:
    //     if (2 == row) {
    //         QFont font;
    //         font.setBold(true);
    //         result = font;
    //     }
    //     break;
    // //
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
    // case Qt::TextAlignmentRole:
    //     result = Qt::AlignCenter;
    //     break;
    // //
    default:
        break;
    }
    //
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
        switch (section) {
        case 0:
            result = "name";
            break;
        case 1:
            result = "identifier";
            break;
        case 2:
            result = "ccy";
            break;
        default:
            break;
        }
    } else if (role == Qt::DisplayRole && orientation == Qt::Vertical) { // V
        return QString("%1").arg(accounts[section].id);
        // switch (section) {
        // case 0:
        //     result = "Welcome";
        //     break;
        // case 1:
        //     result = "Aboard";
        //     break;
        // case 2:
        //     result = "Guys";
        //     break;
        // default:
        //     break;
        // }
    } else {
        // other stuff
    }
    return result;
}

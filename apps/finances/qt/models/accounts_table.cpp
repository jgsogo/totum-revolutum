#include "accounts_table.h"

AccountTableModel::AccountTableModel(QObject* parent) : QAbstractTableModel(parent) {}

int AccountTableModel::rowCount(const QModelIndex& parent) const { return accounts.size(); }
int AccountTableModel::columnCount(const QModelIndex& parent) const { return 5; }

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
            result = account.custodian.second.c_str();
        } else if (column == 1) {
            result = account.name.c_str();
        } else if (column == 2) {
            result = account.identifier.value_or("").c_str();
        } else if (column == 3) {
            result = QString::fromStdString(static_cast<std::string>(account.ccy));
        } else if (column == 4) {
            result = account.type.second.c_str();
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
            result = "custodian";
            break;
        case 1:
            result = "name";
            break;
        case 2:
            result = "identifier";
            break;
        case 3:
            result = "ccy";
            break;
        case 4:
            result = "type";
            break;
        default:
            break;
        }
    } else if (role == Qt::DisplayRole && orientation == Qt::Vertical) { // V
        return QString("%1").arg(accounts[section].id);
    } else {
        // other stuff
    }
    return result;
}

void AccountTableModel::set_accounts(std::vector<finances::accounts::models::Account>&& input) {
    this->beginResetModel();
    accounts.swap(input);
    this->endResetModel();
}

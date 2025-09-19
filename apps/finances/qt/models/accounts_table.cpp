#include "accounts_table.h"

AccountTableModel::AccountTableModel() {}

int AccountTableModel::rowCount(const QModelIndex& parent) const { return 10; }
int AccountTableModel::columnCount(const QModelIndex& parent) const { return 4; }

QVariant AccountTableModel::data(const QModelIndex& index, int role) const {
    QVariant result = QVariant();

    int row = index.row();
    int column = index.column();

    if (!index.isValid() || row >= rowCount() || column >= columnCount()) {
        return result;
    }

    switch (role) {
    case Qt::DisplayRole:
        result = QString("row-%1, col-%2").arg(row).arg(column);
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
            result = "Hello";
            break;
        case 1:
            result = "World";
            break;
        case 2:
            result = ":D";
            break;
        default:
            break;
        }
    } else if (role == Qt::DisplayRole && orientation == Qt::Vertical) { // V
        switch (section) {
        case 0:
            result = "Welcome";
            break;
        case 1:
            result = "Aboard";
            break;
        case 2:
            result = "Guys";
            break;
        default:
            break;
        }
    } else {
        // other stuff
    }
    return result;
}

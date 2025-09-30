#include "movement_type.h"

#include <QDate>
#include <QFont>
#include <QTimer>
#include <magic_enum/magic_enum.hpp>

MovementTypeTableModel::MovementTypeTableModel(utils::libpqxx::ConnectionPool& pool, QObject* parent)
    : QAbstractTableModel(parent), pool{pool} {
    QTimer::singleShot(0, this, SLOT(fetch_all()));
}

int MovementTypeTableModel::rowCount(const QModelIndex&) const { return items.size(); }
int MovementTypeTableModel::columnCount(const QModelIndex&) const { return magic_enum::enum_count<Column>(); }

QVariant MovementTypeTableModel::data(const QModelIndex& index, int role) const {
    QVariant result = QVariant();

    int row = index.row();
    int column_idx = index.column();

    if (!index.isValid() || row >= rowCount() || column_idx >= columnCount()) {
        return result;
    }

    Column column = magic_enum::enum_value<Column>(column_idx);

    switch (role) {
    case Qt::DisplayRole: {
        const auto& movtype = items.at(row);
        switch (column) {
        case Column::ID:
            result = (uint64_t)movtype.id; // FIXME: implement the right conversion
            break;
        case Column::NAME:
            result = movtype.name.c_str();
            break;
        case Column::BREADCRUMB: {
            auto found = this->breadcrumbs.find(movtype.id);
            if (found != this->breadcrumbs.end()) {
                // TODO: Implement this implode as a util
                const char* const delim = " > ";
                std::ostringstream imploded;
                std::copy(found->second.begin(), found->second.end(),
                          std::ostream_iterator<std::string>(imploded, delim));
                result = QString::fromStdString(imploded.str());
            }
        } break;
        }
    } break;
    // case Qt::FontRole:
    //     if ((column == Column::SNAPSHOT) || (column == Column::OPEN) || (column == Column::CLOSE)) {
    //         result = QFont{"Andale Mono"};
    //     }
    //     break;
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

QVariant MovementTypeTableModel::headerData(int section, Qt::Orientation orientation, int role) const {
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
        return QString("%1").arg(items[section].id);
    } else {
        // other stuff
    }
    return result;
}

void MovementTypeTableModel::fetch_all() {
    SPDLOG_DEBUG("MovementTypeTableModel::fetch_all");

    SPDLOG_TRACE(" - fetch all accounts");
    finances::accounts::models::MovementType::Manager manager{pool};
    auto all_items = manager.all();
    if (!all_items) {
        SPDLOG_ERROR("Error refreshing accounts");
        // TODO: Communicate error to user
        return;
    }

    this->beginResetModel();
    this->items = std::move(all_items.value());
    this->endResetModel();

    // We have updated all the accounts, so let's fetch all the snapshots together.
    QTimer::singleShot(0, this, SLOT(fetch_breadcrumbs()));
}

void MovementTypeTableModel::fetch_breadcrumbs() {
    SPDLOG_DEBUG("MovementTypeTableModel::fetch_breadcrumbs");

    finances::accounts::models::MovementType::Manager manager{pool};
    std::map<finances::accounts::models::Id, std::vector<std::string>> breadcrumbs_;
    for (const auto& movtype : items) {
        auto breadcrumb = manager.breadcrumb(movtype.id);
        if (breadcrumb) {
            breadcrumbs_[movtype.id] = std::move(breadcrumb.value());
        }
    }

    this->breadcrumbs = std::move(breadcrumbs_);

    QVector<int> roles = {Qt::DisplayRole};
    QModelIndex topLeft = this->createIndex(0, magic_enum::enum_integer(Column::BREADCRUMB));
    QModelIndex bottomRight = this->createIndex(rowCount(), magic_enum::enum_integer(Column::BREADCRUMB));
    emit dataChanged(topLeft, bottomRight, roles);
}

const finances::accounts::models::MovementType&
MovementTypeTableModel::get_movement_type(finances::accounts::models::Id id) const {
    SPDLOG_TRACE("MovementTypeTableModel::get_movement_type(id={})", id);
    auto found = std::find_if(items.begin(), items.end(), [&id](const auto& item) { return item.id == id; });
    if (found == items.end()) {
        SPDLOG_ERROR(" - Unexpected: MovementType not found!");
        throw std::runtime_error("Details requested for movtype that doesn't exist!");
    }
    return *found;
}

std::optional<std::reference_wrapper<const std::vector<std::string>>>
MovementTypeTableModel::get_breadcrumb(finances::accounts::models::Id id) const {
    auto found = this->breadcrumbs.find(id);
    if (found == breadcrumbs.end()) {
        return {};
    }

    return {found->second};
}

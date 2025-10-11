#include "generic_table_model.h"

#include <QTimer>

using namespace utils::qt::models;

_detail::GenericTableModel::GenericTableModel(utils::libpqxx::ConnectionPool& pool_, QObject* parent)
    : QAbstractTableModel(parent), pool{pool_} {
    QTimer::singleShot(0, this, &_detail::GenericTableModel::refresh_all);
}

void _detail::GenericTableModel::refresh_all() {
    this->beginResetModel();
    this->_refresh_all();
    this->endResetModel();
}

void _detail::GenericTableModel::refresh_one(int row) { this->_refresh_one(row); }

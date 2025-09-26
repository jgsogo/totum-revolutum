#include "account_related.h"

AccountRelatedModel::AccountRelatedModel(utils::libpqxx::ConnectionPool& pool_,
                                         const finances::accounts::models::Account& account_, QObject* parent)
    : QAbstractTableModel(parent), pool{pool_}, account{account_} {}

int AccountRelatedModel::rowCount(const QModelIndex& parent) const { return 0; }

int AccountRelatedModel::columnCount(const QModelIndex& parent) const { return 0; }

QVariant AccountRelatedModel::data(const QModelIndex& index, int role) const {
    QVariant result;
    return result;
}

#pragma once

#include <QAbstractTableModel>

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"

class AccountRelatedModel : public QAbstractTableModel {
    Q_OBJECT
  public:
    AccountRelatedModel(utils::libpqxx::ConnectionPool& pool, const finances::accounts::models::Account&,
                        QObject* parent = nullptr);

    int rowCount(const QModelIndex& parent = QModelIndex()) const override;
    int columnCount(const QModelIndex& parent = QModelIndex()) const override;
    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;

  private:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
};

using AccountSnapshotsModel = AccountRelatedModel;
using AccountMovementsModel = AccountRelatedModel;

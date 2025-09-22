#pragma once

#include <QAbstractTableModel>

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"

// https://stackoverflow.com/a/6791177

// TableView: https://doc.qt.io/qt-6/qtableview.html
// QAbstractTableModel: https://doc.qt.io/qt-6/qabstracttablemodel.html

class AccountTableModel : public QAbstractTableModel {
    Q_OBJECT
  public:
    AccountTableModel(utils::db::ConnectionPool& pool, QObject* parent = nullptr);

    int rowCount(const QModelIndex& parent = QModelIndex()) const override;
    int columnCount(const QModelIndex& parent = QModelIndex()) const override;
    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

  public slots:
    void fetch_all();
    void fetch_snapshots();
    void fetch_snapshot(finances::accounts::models::Id account_id);

  private:
    utils::db::ConnectionPool& pool;
    std::vector<finances::accounts::models::Account> accounts;
    std::vector<std::optional<finances::accounts::models::Snapshot>> snapshots;
};

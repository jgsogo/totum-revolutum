#pragma once

#include <QAbstractTableModel>

#include "libraries/finances/accounts/cpp/models/account.h"

// https://stackoverflow.com/a/6791177

// TableView: https://doc.qt.io/qt-6/qtableview.html
// QAbstractTableModel: https://doc.qt.io/qt-6/qabstracttablemodel.html

class AccountTableModel : public QAbstractTableModel {
  public:
    AccountTableModel(std::vector<finances::accounts::models::Account>&& accounts, QObject* parent = nullptr);

    int rowCount(const QModelIndex& parent = QModelIndex()) const override;
    int columnCount(const QModelIndex& parent = QModelIndex()) const override;
    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;

    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    static AccountTableModel* create_with_all(utils::db::ConnectionPool& pool, QObject* parent = nullptr);

  private:
    std::vector<finances::accounts::models::Account> accounts;
};

#pragma once

#include <QAbstractTableModel>

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/account_holder.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"

class AccountTableModel : public QAbstractTableModel {
    Q_OBJECT
  public:
    enum class Column {
        ID = 0,
        CUSTODIAN = 1,
        NAME = 2,
        IDENTIFIER = 3,
        TYPE = 4,
        SNAPSHOT = 5,
        OPEN = 6,
        CLOSE = 7,
        HOLDERS = 8,
    };

  public:
    AccountTableModel(utils::libpqxx::ConnectionPool& pool, QObject* parent = nullptr);

    int rowCount(const QModelIndex& parent = QModelIndex()) const override;
    int columnCount(const QModelIndex& parent = QModelIndex()) const override;
    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    const finances::accounts::models::Account& get_account(const utils::db::Id& account_id) const;

    const finances::accounts::models::Account& get_account(int row) const;
    const std::vector<finances::accounts::models::AccountHolderWithRoles>& get_holders(int row) const;

  private slots:
    void fetch_all();
    void fetch_snapshots();
    void fetch_account_holders();

  public slots:
    void fetch_snapshot(const utils::db::Id& account_id);

  private:
    utils::libpqxx::ConnectionPool& pool;
    std::vector<finances::accounts::models::Account> accounts;
    std::vector<std::optional<finances::accounts::models::Snapshot>> snapshots;
    std::vector<std::vector<finances::accounts::models::AccountHolderWithRoles>> holders;

    std::map<utils::db::Id, QString> _account_type_breadcrumb;
};

#pragma once

#include <QWidget>

#include "libraries/finances/accounts/cpp/models/transaction.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "apps/finances/qt/models/account_model.h"

#include "apps/finances/qt/tables/accounts.h"
#include "apps/finances/qt/tables/hierarchy_tree_columns.h"
#include "apps/finances/qt/tables/transaction_columns.h"

#include "apps/finances/qt/table_models/accounts.h"
#include "apps/finances/qt/table_models/movement_type.h"
#include "apps/finances/qt/table_models/transactions.h"

class AccountDetailWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AccountDetailWidget(utils::libpqxx::ConnectionPool& pool, AccountsTableModel<AccountColumns>& accounts,
                                 MovementTypesTableModel<HierarchyTreeColumns>& movtypes, const AccountModel&,
                                 QWidget* parent = nullptr);

  private slots:
    void on_new_snapshot(utils::db::Id account_id);

  signals:
    void snapshot_added(utils::db::Id account_id);

  protected:
    void showTransaction(const decltype(finances::accounts::models::Transaction::id)&);

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const AccountModel& account;
    TransactionsForAccountTableModel<TransactionColumns>* transactions_tablemodel;
};

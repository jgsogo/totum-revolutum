#pragma once

#include <QDialog>

#include "libraries/finances/accounts/cpp/models/transaction.h"

#include "apps/finances/qt/table_models/accounts.h"
#include "apps/finances/qt/tables/accounts.h"

class AddTransactionWidget : public QDialog {
    Q_OBJECT

  public:
    AddTransactionWidget(utils::libpqxx::ConnectionPool& pool, AccountsTableModel<AccountColumns>& accounts,
                         QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    utils::libpqxx::ConnectionPool& pool;
    finances::accounts::models::Transaction transaction;
};

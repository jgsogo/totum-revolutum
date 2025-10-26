#pragma once

#include <QDialog>
// #include <QVBoxLayout>

// #include "libraries/finances/accounts/cpp/models/movement.h"

// #include "apps/finances/qt/models/accounts_table.h"
#include "libraries/finances/accounts/cpp/models/transaction.h"

class AddTransactionWidget : public QDialog {
    Q_OBJECT

  public:
    AddTransactionWidget(utils::libpqxx::ConnectionPool& pool, QWidget* parent = nullptr,
                         Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    utils::libpqxx::ConnectionPool& pool;
    finances::accounts::models::Transaction transaction;
};

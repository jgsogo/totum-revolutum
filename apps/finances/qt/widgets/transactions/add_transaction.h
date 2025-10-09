#pragma once

#include <QDialog>
#include <QVBoxLayout>

#include "libraries/finances/accounts/cpp/models/movement.h"

#include "apps/finances/qt/models/accounts_table.h"

class AddTransactionWidget : public QDialog {
    Q_OBJECT

  public:
    AddTransactionWidget(utils::libpqxx::ConnectionPool& pool, AccountTableModel* accounts, QWidget* parent = nullptr,
                         Qt::WindowFlags f = Qt::WindowFlags());

  private slots:
    // Call this slot when the user actually wants to create the transaction
    void add_transaction_clicked();

    // Call this slot when the user wants to add a new movement to the IN or OUT direction
    void add_movement(finances::accounts::models::MovementDirection);

    // Call this slot when the user removes a movement from the IN or OUT direction
    void remove_movement(/* we need some kind of ID here */);

  signals:
    // This signal is emitted after a transaction is created, and it informs about
    // all the accounts that appear in the transaction.
    void new_transaction(std::vector<finances::accounts::models::Id> accounts_ids);

  protected:
    utils::libpqxx::ConnectionPool& pool;
    AccountTableModel* accounts;

    QVBoxLayout* movs_in_layout;
    QVBoxLayout* movs_out_layout;

    // std::vector<AddMovementWidget*> movs_in;
    // std::vector<AddMovementWidget*> movs_out;
};

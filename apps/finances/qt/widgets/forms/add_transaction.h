#pragma once

#include <QDialog>
#include <QDialogButtonBox>
#include <QLabel>
#include <QLineEdit>
#include <QTextEdit>

#include "libraries/finances/accounts/cpp/models/transaction.h"

#include "apps/finances/qt/tables/accounts.h"
#include "apps/finances/qt/tables/hierarchy_tree_columns.h"
#include "apps/finances/qt/tables/movement_columns.h"

#include "apps/finances/qt/table_models/accounts.h"
#include "apps/finances/qt/table_models/movement_type.h"
#include "apps/finances/qt/table_models/movements.h"

class AddTransactionWidget : public QDialog {
    Q_OBJECT

  public:
    AddTransactionWidget(utils::libpqxx::ConnectionPool& pool, AccountsTableModel<AccountColumns>& accounts,
                         MovementTypesTableModel<HierarchyTreeColumns>& movtypes, QWidget* parent = nullptr,
                         Qt::WindowFlags f = Qt::WindowFlags());

    bool is_valid() const;

  public slots:
    void on_new_movement(MovementModel movement);

  private slots:
    // User wants to add a transaction to the DB
    void add_transaction_clicked();

  signals:
    // After creating a transaction, we notify which accounts have now new movements
    void new_movement(utils::db::Id account_id);

  protected:
    utils::libpqxx::ConnectionPool& pool;
    finances::accounts::models::Transaction transaction;
    MovementsForTransactionTableModel<MovementColumns>* movements;

    QLineEdit* transaction_title;
    QTextEdit* transaction_description;

    finances::accounts::models::Money money_in;
    finances::accounts::models::Money money_out;

    QLabel* money_in_label;
    QLabel* money_out_label;

    QDialogButtonBox* buttonBox;
};

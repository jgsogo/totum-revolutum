#pragma once

#include <QDialog>
#include <QDialogButtonBox>
#include <QLabel>

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

  public slots:
    void on_new_movement(MovementModel movement);

  protected:
    utils::libpqxx::ConnectionPool& pool;
    finances::accounts::models::Transaction transaction;
    MovementsForTransactionTableModel<MovementColumns>* movements;

    finances::accounts::models::Money money_in;
    finances::accounts::models::Money money_out;

    QLabel* money_in_label;
    QLabel* money_out_label;

    QDialogButtonBox* buttonBox;
};

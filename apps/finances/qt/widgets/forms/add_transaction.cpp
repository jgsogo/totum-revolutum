#include "add_transaction.h"

#include <QDialogButtonBox>
#include <QHeaderView>
#include <QLabel>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

#include "apps/finances/qt/table_models/movements.h"
#include "apps/finances/qt/tables/movement_columns.h"

AddTransactionWidget::AddTransactionWidget(utils::libpqxx::ConnectionPool& pool, QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool} {

    // Models
    MovementsForTransactionTableModel<MovementColumns>* model =
        new MovementsForTransactionTableModel<MovementColumns>(transaction, pool, this);

    // Components
    QTableView* table_view = new QTableView(this);
    {
        QSortFilterProxyModel* sort_filter = new QSortFilterProxyModel(this);
        sort_filter->setSourceModel(model);
        sort_filter->sort(magic_enum::enum_integer(MovementColumns::DATE_VALUE), Qt::DescendingOrder);

        table_view->setModel(sort_filter);
        table_view->setSortingEnabled(false);
        table_view->hideColumn(magic_enum::enum_integer(MovementColumns::ID));
        table_view->verticalHeader()->hide();
        table_view->horizontalHeader()->setSectionResizeMode(QHeaderView::ResizeToContents);

        table_view->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        table_view->resizeColumnsToContents();
        table_view->resizeRowsToContents();
    }

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &QDialog::accept);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(new QLabel(tr("Add new transaction")));
    mainLayout->addWidget(table_view);
    mainLayout->addWidget(buttonBox);

    this->setLayout(mainLayout);
}

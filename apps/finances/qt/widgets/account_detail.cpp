#include "account_detail.h"

#include <QConcatenateTablesProxyModel>
#include <QHeaderView>
#include <QLabel>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

AccountDetailWidget::AccountDetailWidget(const finances::accounts::models::Account& account_,
                                         AccountRelatedSnapshotsAsMovementsModel* snapshots_model,
                                         AccountRelatedMovementsModel* movements_model, QWidget* parent)
    : QWidget(parent), account{account_} {

    // Models
    QConcatenateTablesProxyModel* model = new QConcatenateTablesProxyModel(this);
    model->addSourceModel(snapshots_model);
    model->addSourceModel(movements_model);

    // Components
    QSortFilterProxyModel* sort_filter = new QSortFilterProxyModel(this);
    sort_filter->setSourceModel(model);
    sort_filter->sort(magic_enum::enum_integer(MovementColumn::DATE_VALUE), Qt::DescendingOrder);

    QTableView* table_view = new QTableView(this);
    table_view->setModel(sort_filter);
    table_view->setSortingEnabled(false);
    table_view->hideColumn(magic_enum::enum_integer(MovementColumn::ID));
    table_view->verticalHeader()->hide();

    QLabel* name = new QLabel(QString::fromStdString(account.name));

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(name);
    mainLayout->addWidget(table_view);

    this->setLayout(mainLayout);
}

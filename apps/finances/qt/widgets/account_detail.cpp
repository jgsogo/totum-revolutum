#include "account_detail.h"

#include <QConcatenateTablesProxyModel>
#include <QHeaderView>
#include <QLabel>
#include <QPushButton>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

#include "account_add_snapshot.h"

AccountDetailWidget::AccountDetailWidget(const finances::accounts::models::Account& account_,
                                         AccountRelatedModelBase* snapshots_model,
                                         AccountRelatedModelBase* movements_model, QWidget* parent)
    : QWidget(parent), account{account_} {

    // Models
    QConcatenateTablesProxyModel* model = new QConcatenateTablesProxyModel(this);
    model->addSourceModel(snapshots_model);
    model->addSourceModel(movements_model);

    // Components
    QLabel* name = new QLabel(QString::fromStdString(account.name));

    QSortFilterProxyModel* sort_filter = new QSortFilterProxyModel(this);
    sort_filter->setSourceModel(model);
    sort_filter->sort(magic_enum::enum_integer(MovementColumn::DATE_VALUE), Qt::DescendingOrder);

    QTableView* table_view = new QTableView(this);
    table_view->setModel(sort_filter);
    table_view->setSortingEnabled(false);
    table_view->hideColumn(magic_enum::enum_integer(MovementColumn::ID));
    table_view->verticalHeader()->hide();

    // - popup - add snapshot
    AddSnapshotWidget* popup_add_snapshot = new AddSnapshotWidget(this);
    popup_add_snapshot->setModal(true);
    popup_add_snapshot->setSizeGripEnabled(true);
    connect(popup_add_snapshot, &AddSnapshotWidget::new_snapshot, this, &AccountDetailWidget::on_new_snapshot);

    QPushButton* bt_add_snapshot = new QPushButton(tr("Add snapshot"), this);
    bt_add_snapshot->setDisabled(account.is_numerable);
    connect(bt_add_snapshot, &QPushButton::clicked, popup_add_snapshot, &QDialog::exec);

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(name);
    mainLayout->addWidget(bt_add_snapshot);
    mainLayout->addWidget(table_view);

    this->setLayout(mainLayout);
}

void AccountDetailWidget::on_new_snapshot(Snapshot snapshot) {
    SPDLOG_DEBUG("AccountDetailWidget::on_new_snapshot(date={}, amount={})");

    // TODO: Add to database and notify DB channel!
    //       The Notifier will do its work updating all the models.
}

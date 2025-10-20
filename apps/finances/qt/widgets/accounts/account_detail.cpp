#include "account_detail.h"

#include <QConcatenateTablesProxyModel>
#include <QHeaderView>
#include <QLabel>
#include <QPushButton>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

#include "apps/finances/qt/table_models/movements.h"
#include "apps/finances/qt/table_models/snapshots.h"
#include "apps/finances/qt/tables/movement_columns.h"
#include "non_numerable/add_snapshot.h"
#include "numerable/add_snapshot.h"

AccountDetailWidget::AccountDetailWidget(utils::libpqxx::ConnectionPool& pool_, const AccountModel& account_,
                                         QWidget* parent)
    : QWidget(parent), pool{pool_}, account{account_} {

    // Models
    QConcatenateTablesProxyModel* model = new QConcatenateTablesProxyModel(this);

    // - movements
    {
        MovementsForAccountTableModel<MovementColumns>* movements_tablemodel =
            new MovementsForAccountTableModel<MovementColumns>(account.account, pool, this);
        model->addSourceModel(movements_tablemodel);
    }

    // - snapshots
    {
        if (account.account.is_numerable) {
            NumerableSnapshotsTableModel<MovementColumns>* snapshots_tablemodel =
                new NumerableSnapshotsTableModel<MovementColumns>(account.account, pool, this);
            connect(this, &AccountDetailWidget::snapshot_added, snapshots_tablemodel,
                    &utils::qt::models::_detail::GenericTableModel::refresh_all);

            model->addSourceModel(snapshots_tablemodel);
        } else {
            SnapshotsTableModel<MovementColumns>* snapshots_tablemodel =
                new SnapshotsTableModel<MovementColumns>(account.account, pool, this);
            connect(this, &AccountDetailWidget::snapshot_added, snapshots_tablemodel,
                    &utils::qt::models::_detail::GenericTableModel::refresh_all);

            model->addSourceModel(snapshots_tablemodel);
        }
    }

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
    }

    // - popup - add snapshot
    QPushButton* bt_add_snapshot = new QPushButton(tr("Add snapshot"), this);
    if (account.account.is_numerable) {
        AddSnapshotNumerableWidget* popup_add_snapshot = new AddSnapshotNumerableWidget(pool, account.account, this);
        popup_add_snapshot->setModal(true);
        popup_add_snapshot->setSizeGripEnabled(true);
        connect(popup_add_snapshot, &AddSnapshotNumerableWidget::new_snapshot, this,
                &AccountDetailWidget::on_new_snapshot);

        connect(bt_add_snapshot, &QPushButton::clicked, popup_add_snapshot, &QDialog::open);
    } else {
        AddSnapshotNonNumerableWidget* popup_add_snapshot =
            new AddSnapshotNonNumerableWidget(pool, account.account, this);
        popup_add_snapshot->setModal(true);
        popup_add_snapshot->setSizeGripEnabled(true);
        connect(popup_add_snapshot, &AddSnapshotNonNumerableWidget::new_snapshot, this,
                &AccountDetailWidget::on_new_snapshot);

        connect(bt_add_snapshot, &QPushButton::clicked, popup_add_snapshot, &QDialog::open);
    }

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(new QLabel(QString::fromStdString(account.account.name)));
    mainLayout->addWidget(bt_add_snapshot);
    mainLayout->addWidget(table_view);

    this->setLayout(mainLayout);
}

void AccountDetailWidget::on_new_snapshot(utils::db::Id account_id) {
    SPDLOG_DEBUG("AccountDetailWidget::on_new_snapshot(account_id={})", account_id);
    assert(account_id == account.id);
    emit snapshot_added(account_id);
}

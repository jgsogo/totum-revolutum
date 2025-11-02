#include "account_detail.h"

#include <QConcatenateTablesProxyModel>
#include <QHeaderView>
#include <QLabel>
#include <QPushButton>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

#include "apps/finances/qt/metatypes/types.h"
#include "apps/finances/qt/table_models/movements.h"
#include "apps/finances/qt/table_models/snapshots.h"
#include "apps/finances/qt/tables/movement_columns.h"
#include "apps/finances/qt/widgets/forms/add_transaction.h"
#include "apps/finances/qt/widgets/transactions/transaction_detail.h"

#include "non_numerable/add_snapshot.h"
#include "numerable/add_snapshot.h"

AccountDetailWidget::AccountDetailWidget(utils::libpqxx::ConnectionPool& pool_,
                                         AccountsTableModel<AccountColumns>& accounts_, const AccountModel& account_,
                                         QWidget* parent)
    : QWidget(parent), pool{pool_}, account{account_} {

    // Models
    QConcatenateTablesProxyModel* model = new QConcatenateTablesProxyModel(this);
    transactions_tablemodel = new TransactionsForAccountTableModel<TransactionColumns>(account.account, pool, this);

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

        // When the user double-click in a row, if it is a movement, we want to open the corresponding transaction
        // window
        connect(table_view, &QTableView::doubleClicked, [sort_filter, this](const QModelIndex& index) {
            SPDLOG_DEBUG("User clicked on row={}", index.row());
            auto transaction_column = index.siblingAtColumn(magic_enum::enum_integer(MovementColumns::TRANSACTION_ID));
            QVariant transaction_id_variant = sort_filter->data(transaction_column);
            utils::db::Id transaction_id = transaction_id_variant.value<utils::db::Id>();
            if (utils::db::is_null(transaction_id)) {
                SPDLOG_DEBUG("No transaction id associated to row {}, or value cannot be converted into utils::db::Id",
                             index.row());
                return;
            }
            this->showTransaction(transaction_id);
        });
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

    // - popup - add transaction
    QPushButton* bt_add_transaction = new QPushButton(tr("Add transaction"), this);
    {
        AddTransactionWidget* popup_add_transaction = new AddTransactionWidget(pool, accounts_, this);
        popup_add_transaction->setModal(true);
        popup_add_transaction->setSizeGripEnabled(true);
        // connect(popup_add_transaction, &AddSnapshotNumerableWidget::new_snapshot, this,
        //         &AccountDetailWidget::on_new_snapshot);
        connect(bt_add_transaction, &QPushButton::clicked, popup_add_transaction, &QDialog::open);
    }

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(new QLabel(QString::fromStdString(account.account.name)));
    mainLayout->addWidget(bt_add_snapshot);
    mainLayout->addWidget(bt_add_transaction);
    mainLayout->addWidget(table_view);

    this->setLayout(mainLayout);
}

void AccountDetailWidget::on_new_snapshot(utils::db::Id account_id) {
    SPDLOG_DEBUG("AccountDetailWidget::on_new_snapshot(account_id={})", account_id);
    assert(account_id == account.id);
    emit snapshot_added(account_id);
}

void AccountDetailWidget::showTransaction(const decltype(finances::accounts::models::Transaction::id)& transaction_id) {
    SPDLOG_DEBUG("AccountDetailWidget::showTransaction(transaction_id={})", transaction_id);

    auto transaction_expected = transactions_tablemodel->get(transaction_id);
    if (!transaction_expected) {
        SPDLOG_ERROR("Transaction with id '{}' not found in this account", transaction_id);
        // TODO: Notify error to user
        return;
    }

    const finances::accounts::models::Transaction& transaction = transaction_expected.value();
    TransactionDetailWidget* transaction_detail = new TransactionDetailWidget(pool, transaction, this);
    transaction_detail->setModal(true);
    transaction_detail->setSizeGripEnabled(true);
    transaction_detail->open();

    // SPDLOG_TRACE(" - this is transaction_id {}", transaction_id);
    //         utils::db::ModelData<finances::accounts::models::Transaction>::Manager transactions_manager{pool};
    //         auto transaction_expected = transactions_manager.get(transaction_id);
    //         if (!transaction_expected) {
    //             SPDLOG_ERROR("Failed to get transaction with id {} (corresponding to row {}): {}", transaction_id,
    //                          index.row(), transaction_expected.error());
    //             return;
    //         }

    //         // FIXME: Here I'm using a temporal Transaction and passing a reference!!!!
    //         SPDLOG_WARN("STOP! We cannot create a TransactionDetailWidget using a reference!");
    //         // const finances::accounts::models::Transaction& transaction = transaction_expected.value();
    //         // TransactionDetailWidget* transaction_detail =
    //         // new TransactionDetailWidget(pool, transaction, this);
    //         // transaction_detail->setModal(true);
    //         // transaction_detail->setSizeGripEnabled(true);
    //         // transaction_detail->open();
}

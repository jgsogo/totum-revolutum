#include "account_detail.h"

#include <QConcatenateTablesProxyModel>
#include <QHeaderView>
#include <QLabel>
#include <QPushButton>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

#include "account_add_snapshot.h"
#include "apps/finances/qt/models/account_related.h"

AccountDetailWidget::AccountDetailWidget(utils::libpqxx::ConnectionPool& pool_,
                                         const finances::accounts::models::Account& account_, QWidget* parent)
    : QWidget(parent), pool{pool_}, account{account_} {

    AccountRelatedModelBase* snapshots_model =
        new AccountRelatedModel<finances::accounts::models::Snapshot, MovementColumn>(pool, account, this);
    connect(this, &AccountDetailWidget::snapshot_added, snapshots_model, &AccountRelatedModelBase::fetch_all);
    AccountRelatedModelBase* movements_model =
        new AccountRelatedModel<finances::accounts::models::Movement, MovementColumn>(pool, account, this);

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

void AccountDetailWidget::on_new_snapshot(Snapshot2Decs snapshot) {
    SPDLOG_DEBUG("AccountDetailWidget::on_new_snapshot(date={}, amount={})", snapshot.date().toString().toStdString(),
                 dec::toString(snapshot.amount(), dec::decimal_format{','}));

    auto qt_date = snapshot.date();
    auto qt_amount = snapshot.amount();

    utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
                                                   date::month{static_cast<unsigned int>(qt_date.month())},
                                                   date::day{static_cast<unsigned int>(qt_date.day())}}};
    finances::accounts::models::Amount amount{qt_amount};

    finances::accounts::models::SnapshotManager manager{pool};
    auto r = manager.create(account.id, std::move(date), std::move(amount));
    if (!r) {
        SPDLOG_ERROR("Error adding snapshot to account");
        // TODO: Communicate error to user
        return;
    }

    emit snapshot_added(account.id);
}

#include "account_numerable_detail.h"

#include <QConcatenateTablesProxyModel>
#include <QHeaderView>
#include <QLabel>
#include <QPushButton>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

#include "libraries/finances/accounts/cpp/models/types/money.h"
#include "libraries/finances/investments/cpp/models/movement_dividend.h"
#include "libraries/finances/investments/cpp/models/movement_numerable.h"
#include "libraries/finances/investments/cpp/models/snapshot_numerable.h"

#include "account_add_snapshot_numerable.h"
#include "apps/finances/qt/models/account_related.h"

enum class MovementNumerableColumn {
    ID = 0,
    DATE_VALUE = 1,
    MOVE_TYPE = 2,
    TRANSACTION = 3,
    DIRECTION = 4,
    AMOUNT = 5,
    QUANTITY = 6,
    UNIT_VALUE = 7,
};

AccountNumerableDetailWidget::AccountNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool_,
                                                           const finances::accounts::models::Account& account_,
                                                           QWidget* parent)
    : QWidget(parent), pool{pool_}, account{account_} {

    AccountRelatedModelBase* snapshots_model =
        new AccountRelatedModel<finances::investments::models::SnapshotNumerable, MovementNumerableColumn>(
            pool, account, this);
    connect(this, &AccountNumerableDetailWidget::snapshot_added, snapshots_model, &AccountRelatedModelBase::fetch_all);
    AccountRelatedModelBase* movements_model =
        new AccountRelatedModel<finances::investments::models::MovementNumerable, MovementNumerableColumn>(
            pool, account, this);
    AccountRelatedModelBase* dividends_model =
        new AccountRelatedModel<finances::investments::models::MovementDividend, MovementNumerableColumn>(pool, account,
                                                                                                          this);

    // Models
    QConcatenateTablesProxyModel* model = new QConcatenateTablesProxyModel(this);
    model->addSourceModel(snapshots_model);
    model->addSourceModel(movements_model);
    model->addSourceModel(dividends_model);

    // Components
    QLabel* name = new QLabel(QString::fromStdString(account.name));

    QSortFilterProxyModel* sort_filter = new QSortFilterProxyModel(this);
    sort_filter->setSourceModel(model);
    sort_filter->sort(magic_enum::enum_integer(MovementNumerableColumn::DATE_VALUE), Qt::DescendingOrder);

    QTableView* table_view = new QTableView(this);
    table_view->setModel(sort_filter);
    table_view->setSortingEnabled(false);
    table_view->hideColumn(magic_enum::enum_integer(MovementNumerableColumn::ID));
    table_view->verticalHeader()->hide();

    // - popup - add snapshot
    AddSnapshotNumerableWidget* popup_add_snapshot = new AddSnapshotNumerableWidget(this);
    popup_add_snapshot->setModal(true);
    popup_add_snapshot->setSizeGripEnabled(true);
    connect(popup_add_snapshot, &AddSnapshotNumerableWidget::new_snapshot, this,
            &AccountNumerableDetailWidget::on_new_snapshot);

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

void AccountNumerableDetailWidget::on_new_snapshot(SnapshotNumerable snapshot) {
    SPDLOG_DEBUG("AccountNumerableDetailWidget::on_new_snapshot(numerable)");

    // auto qt_date = snapshot.date();
    // utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
    //                                                date::month{static_cast<unsigned int>(qt_date.month())},
    //                                                date::day{static_cast<unsigned int>(qt_date.day())}}};

    // auto quantity = snapshot.quantity();
    // auto unit_value = snapshot.unit_value();

    // TODO: Not implemented
    SPDLOG_ERROR("Not implemented!");

    // finances::accounts::models::SnapshotNumerableManager manager{pool};
    // auto r = manager.create(account.id, std::move(date), std::move(amount));
    // if (!r) {
    //     SPDLOG_ERROR("Error adding snapshot to account");
    //     // TODO: Communicate error to user
    //     return;
    // }

    emit snapshot_added(account.id);
}

#include "account_detail.h"

#include <QConcatenateTablesProxyModel>
#include <QHeaderView>
#include <QLabel>
#include <QPushButton>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

#include "apps/finances/qt/models/account_related.h"
#include "apps/finances/qt/table_models/movements.h"
#include "apps/finances/qt/table_models/snapshots.h"
#include "apps/finances/qt/tables/account_movements.h"
#include "libraries/finances/accounts/cpp/models/types/money.h"
#include "libraries/finances/investments/cpp/models/movement_dividend.h"
#include "libraries/finances/investments/cpp/models/movement_numerable.h"
#include "libraries/finances/investments/cpp/models/snapshot_numerable.h"

#include "add_snapshot.h"

// enum class MovementNumerableColumn {
//     ID = 0,
//     DATE_VALUE = 1,
//     MOVE_TYPE = 2,
//     TRANSACTION = 3,
//     DIRECTION = 4,
//     AMOUNT = 5,
//     QUANTITY = 6,
//     UNIT_VALUE = 7,
// };

AccountNumerableDetailWidget::AccountNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool_,
                                                           const AccountModel& account_,
                                                           const MovementTypeTableModel* movtype_model_,
                                                           QWidget* parent)

    : AccountDetailWidget(pool_, account_, movtype_model_, parent) {

    NumerableSnapshotsTableModel<AccountMovementsColumns>* snapshots_tablemodel =
        new NumerableSnapshotsTableModel<AccountMovementsColumns>(account.account, pool, this);
    connect(this, &AccountNumerableDetailWidget::snapshot_added, snapshots_tablemodel,
            &utils::qt::models::_detail::GenericTableModel::refresh_all);
    MovementsTableModel<AccountMovementsColumns>* movements_tablemodel =
        new MovementsTableModel<AccountMovementsColumns>(account.account, pool, this);

    // AccountRelatedModelBase* snapshots_model =
    //     new AccountRelatedModel<finances::investments::models::SnapshotNumerable, MovementNumerableColumn>(
    //         pool, account.account, nullptr, this);
    // connect(this, &AccountNumerableDetailWidget::snapshot_added, snapshots_model,
    // &AccountRelatedModelBase::fetch_all); AccountRelatedModelBase* movements_model =
    //     new AccountRelatedModel<finances::investments::models::MovementNumerable, MovementNumerableColumn>(
    //         pool, account.account, movtype_model, this);
    // AccountRelatedModelBase* dividends_model =
    //     new AccountRelatedModel<finances::investments::models::MovementDividend, MovementNumerableColumn>(
    //         pool, account.account, movtype_model, this);

    // Models
    QConcatenateTablesProxyModel* model = new QConcatenateTablesProxyModel(this);
    model->addSourceModel(snapshots_tablemodel);
    model->addSourceModel(movements_tablemodel);
    // model->addSourceModel(dividends_model);

    // Components
    QLabel* name = new QLabel(QString::fromStdString(account.account.name));

    QSortFilterProxyModel* sort_filter = new QSortFilterProxyModel(this);
    sort_filter->setSourceModel(model);
    sort_filter->sort(magic_enum::enum_integer(AccountMovementsColumns::DATE_VALUE), Qt::DescendingOrder);

    QTableView* table_view = new QTableView(this);
    table_view->setModel(sort_filter);
    table_view->setSortingEnabled(false);
    table_view->hideColumn(magic_enum::enum_integer(AccountMovementsColumns::ID));
    table_view->verticalHeader()->hide();
    table_view->horizontalHeader()->setSectionResizeMode(QHeaderView::ResizeToContents);

    // - popup - add snapshot
    AddSnapshotNumerableWidget* popup_add_snapshot = new AddSnapshotNumerableWidget(pool, account.account, this);
    popup_add_snapshot->setModal(true);
    popup_add_snapshot->setSizeGripEnabled(true);
    connect(popup_add_snapshot, &AddSnapshotNumerableWidget::new_snapshot, this,
            &AccountNumerableDetailWidget::on_new_snapshot);

    QPushButton* bt_add_snapshot = new QPushButton(tr("Add snapshot"), this);
    connect(bt_add_snapshot, &QPushButton::clicked, popup_add_snapshot, &QDialog::open);

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(name);
    mainLayout->addWidget(bt_add_snapshot);
    mainLayout->addWidget(table_view);

    this->setLayout(mainLayout);
}

void AccountNumerableDetailWidget::on_new_snapshot(utils::db::Id account_id) {
    SPDLOG_DEBUG("AccountNumerableDetailWidget::on_new_snapshot(account_id={})", account_id);
    assert(account_id == account.id);
    emit snapshot_added(account_id);
}

// template <typename TModel> struct DataDispatcher<TModel, MovementNumerableColumn, Qt::FontRole> {
//     static QVariant data(const AccountRelatedModelBase&, const TModel&, MovementNumerableColumn column) {
//         if ((column == MovementNumerableColumn::DATE_VALUE) || (column == MovementNumerableColumn::AMOUNT) ||
//             (column == MovementNumerableColumn::QUANTITY) || (column == MovementNumerableColumn::UNIT_VALUE)) {
//             return QVariant{QFont{"Andale Mono"}};
//         }
//         return QVariant{};
//     }
// };

// template <typename TColumn>
// struct DataDispatcher<finances::investments::models::SnapshotNumerable, TColumn, Qt::BackgroundRole> {
//     static QVariant data(const AccountRelatedModelBase&, const finances::investments::models::SnapshotNumerable&,
//                          TColumn column) {
//         return QVariant{QColor(255, 255, 40)};
//     }
// };

// template <typename TColumn>
// struct DataDispatcher<finances::investments::models::MovementDividend, TColumn, Qt::BackgroundRole> {
//     static QVariant data(const AccountRelatedModelBase&, const finances::investments::models::MovementDividend&,
//                          TColumn column) {
//         return QVariant{QColor(230, 249, 255)};
//     }
// };

// template <>
// QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementNumerableColumn, Qt::DisplayRole>::data(
//     const AccountRelatedModelBase& account, const finances::accounts::models::Snapshot& item,
//     MovementNumerableColumn column) {
//     QVariant result = QVariant();

//     switch (column) {
//     case MovementNumerableColumn::ID:
//         result = QString::fromStdString(std::format("{}", item.id)); // FIXME: implement the right conversion
//         break;
//     case MovementNumerableColumn::DATE_VALUE:
//         result = QDate{int(item.date_value.year()), static_cast<int>(unsigned(item.date_value.month())),
//                        static_cast<int>(unsigned(item.date_value.day()))}
//                      .toString("yyyy-MM-dd");
//         break;
//     case MovementNumerableColumn::AMOUNT: {
//         auto amount_money = finances::accounts::models::Money{item.amount, account.account.ccy};
//         result = QString::fromStdString(static_cast<std::string>(amount_money));
//     } break;
//     case MovementNumerableColumn::QUANTITY:
//     case MovementNumerableColumn::UNIT_VALUE: {
//         SPDLOG_ERROR("Never get here! It should be already handled by the SnapshotNumerable");
//     } break;
//     // Snapshots doesn't have these fields
//     case MovementNumerableColumn::TRANSACTION:
//     case MovementNumerableColumn::MOVE_TYPE:
//     case MovementNumerableColumn::DIRECTION:
//         break;
//     }
//     return result;
// }

// template <>
// QVariant
// DataDispatcher<finances::investments::models::SnapshotNumerable, MovementNumerableColumn, Qt::DisplayRole>::data(
//     const AccountRelatedModelBase& account, const finances::investments::models::SnapshotNumerable& item,
//     MovementNumerableColumn column) {
//     QVariant result = QVariant();

//     switch (column) {
//     case MovementNumerableColumn::QUANTITY: {
//         result = QString::fromStdString(static_cast<std::string>(item.quantity));
//     } break;
//     case MovementNumerableColumn::UNIT_VALUE: {
//         auto unit_value_money = finances::accounts::models::Money{item.unit_value, account.account.ccy};
//         result = QString::fromStdString(static_cast<std::string>(unit_value_money));
//     } break;
//     // Forward all the others to the underlying Snapshot type
//     case MovementNumerableColumn::ID:
//     case MovementNumerableColumn::DATE_VALUE:
//     case MovementNumerableColumn::AMOUNT:
//     case MovementNumerableColumn::TRANSACTION:
//     case MovementNumerableColumn::MOVE_TYPE:
//     case MovementNumerableColumn::DIRECTION: {
//         result = ::DataDispatcher<finances::accounts::models::Snapshot, MovementNumerableColumn,
//         Qt::DisplayRole>::data(
//             account, item.snapshot, column);
//         break;
//     }
//     }
//     return result;
// }

// template <>
// QVariant DataDispatcher<finances::accounts::models::Movement, MovementNumerableColumn, Qt::DisplayRole>::data(
//     const AccountRelatedModelBase& account, const finances::accounts::models::Movement& item,
//     MovementNumerableColumn column) {
//     QVariant result = QVariant();
//     switch (column) {
//     case MovementNumerableColumn::ID:
//         result = QString::fromStdString(std::format("{}", item.id)); // FIXME: implement the right conversion
//         break;
//     case MovementNumerableColumn::DATE_VALUE:
//         result = QDate{int(item.date_value.year()), static_cast<int>(unsigned(item.date_value.month())),
//                        static_cast<int>(unsigned(item.date_value.day()))}
//                      .toString("yyyy-MM-dd");
//         break;
//     case MovementNumerableColumn::AMOUNT: {
//         auto amount_money = finances::accounts::models::Money{item.amount, account.account.ccy};
//         result = QString::fromStdString(static_cast<std::string>(amount_money));
//     } break;
//     case MovementNumerableColumn::TRANSACTION:
//         result = item.transaction.second.c_str();
//         break;
//     case MovementNumerableColumn::MOVE_TYPE: {
//         auto breadcrumb = account.movtype_model->get_breadcrumb(item.type.first);
//         if (breadcrumb) {
//             // FIXME: We are doing this in multiple places
//             QString q_breadcrumb;
//             for (const auto& it : breadcrumb.value().get()) {
//                 q_breadcrumb.append(it.second.c_str());
//                 q_breadcrumb.append(" > ");
//             }
//             q_breadcrumb.append(item.type.second.c_str());

//             result = q_breadcrumb;
//         } else {
//             result = item.type.second.c_str();
//         }
//     } break;
//     case MovementNumerableColumn::DIRECTION:
//         result = QString::fromStdString(std::string(magic_enum::enum_name(item.direction)));
//         break;

//     case MovementNumerableColumn::QUANTITY:
//     case MovementNumerableColumn::UNIT_VALUE: {
//         SPDLOG_ERROR("Never get here! It should be already handled by the MovementNumerable");
//     } break;
//     }
//     return result;
// }

// template <>
// QVariant
// DataDispatcher<finances::investments::models::MovementNumerable, MovementNumerableColumn, Qt::DisplayRole>::data(
//     const AccountRelatedModelBase& account, const finances::investments::models::MovementNumerable& item,
//     MovementNumerableColumn column) {
//     QVariant result = QVariant();
//     switch (column) {
//     case MovementNumerableColumn::QUANTITY: {
//         result = QString::fromStdString(static_cast<std::string>(item.quantity));
//     } break;
//     case MovementNumerableColumn::UNIT_VALUE: {
//         auto money = finances::accounts::models::Money{item.unit_value, account.account.ccy};
//         result = QString::fromStdString(static_cast<std::string>(money));
//     } break;
//         // Forward all the others to the underlying Movement type
//     case MovementNumerableColumn::ID:
//     case MovementNumerableColumn::DATE_VALUE:
//     case MovementNumerableColumn::AMOUNT:
//     case MovementNumerableColumn::TRANSACTION:
//     case MovementNumerableColumn::MOVE_TYPE:
//     case MovementNumerableColumn::DIRECTION: {
//         result = ::DataDispatcher<finances::accounts::models::Movement, MovementNumerableColumn,
//         Qt::DisplayRole>::data(
//             account, item.movement, column);
//         break;
//     }
//     }
//     return result;
// }

// template <>
// QVariant
// DataDispatcher<finances::investments::models::MovementDividend, MovementNumerableColumn, Qt::DisplayRole>::data(
//     const AccountRelatedModelBase& account, const finances::investments::models::MovementDividend& item,
//     MovementNumerableColumn column) {
//     QVariant result = QVariant();
//     switch (column) {
//     case MovementNumerableColumn::QUANTITY: {
//         result = QString::fromStdString(static_cast<std::string>(item.snapshot_data.value().second));
//     } break;
//     case MovementNumerableColumn::UNIT_VALUE: {
//         auto money = finances::accounts::models::Money{item.unit_value, account.account.ccy};
//         result = QString::fromStdString(static_cast<std::string>(money));
//     } break;
//         // Forward all the others to the underlying Movement type
//     case MovementNumerableColumn::ID:
//     case MovementNumerableColumn::DATE_VALUE:
//     case MovementNumerableColumn::AMOUNT:
//     case MovementNumerableColumn::TRANSACTION:
//     case MovementNumerableColumn::MOVE_TYPE:
//     case MovementNumerableColumn::DIRECTION: {
//         result = ::DataDispatcher<finances::accounts::models::Movement, MovementNumerableColumn,
//         Qt::DisplayRole>::data(
//             account, item.movement, column);
//         break;
//     }
//     }
//     return result;
// }

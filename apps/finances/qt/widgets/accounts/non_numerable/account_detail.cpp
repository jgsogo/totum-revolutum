#include "account_detail.h"

#include <QConcatenateTablesProxyModel>
#include <QHeaderView>
#include <QLabel>
#include <QPushButton>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

#include "apps/finances/qt/models/account_related.h"
#include "libraries/finances/accounts/cpp/models/movement.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"
#include "libraries/finances/accounts/cpp/models/types/money.h"

#include "add_snapshot.h"

enum class MovementColumn {
    ID = 0,
    DATE_VALUE = 1,
    MOVE_TYPE = 2,
    TRANSACTION = 3,
    DIRECTION = 4,
    AMOUNT = 5,
};

AccountNonNumerableDetailWidget::AccountNonNumerableDetailWidget(utils::libpqxx::ConnectionPool& pool_,
                                                                 const finances::accounts::models::Account& account_,
                                                                 const MovementTypeTableModel* movtype_model_,
                                                                 QWidget* parent)
    : AccountDetailWidget(pool_, account_, movtype_model_, parent) {

    AccountRelatedModelBase* snapshots_model =
        new AccountRelatedModel<finances::accounts::models::Snapshot, MovementColumn>(pool, account, nullptr, this);
    connect(this, &AccountNonNumerableDetailWidget::snapshot_added, snapshots_model,
            &AccountRelatedModelBase::fetch_all);
    AccountRelatedModelBase* movements_model =
        new AccountRelatedModel<finances::accounts::models::Movement, MovementColumn>(pool, account, movtype_model,
                                                                                      this);

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
    table_view->horizontalHeader()->setSectionResizeMode(QHeaderView::ResizeToContents);

    // - popup - add snapshot
    AddSnapshotNonNumerableWidget* popup_add_snapshot = new AddSnapshotNonNumerableWidget(this);
    popup_add_snapshot->setModal(true);
    popup_add_snapshot->setSizeGripEnabled(true);
    connect(popup_add_snapshot, &AddSnapshotNonNumerableWidget::new_snapshot, this,
            &AccountNonNumerableDetailWidget::on_new_snapshot);

    QPushButton* bt_add_snapshot = new QPushButton(tr("Add snapshot"), this);
    connect(bt_add_snapshot, &QPushButton::clicked, popup_add_snapshot, &QDialog::exec);

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(name);
    mainLayout->addWidget(bt_add_snapshot);
    mainLayout->addWidget(table_view);

    this->setLayout(mainLayout);
}

void AccountNonNumerableDetailWidget::on_new_snapshot(SnapshotNonNumerable snapshot) {
    SPDLOG_DEBUG("AccountNonNumerableDetailWidget::on_new_snapshot(non-numerable)");

    auto qt_date = snapshot.date();
    utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
                                                   date::month{static_cast<unsigned int>(qt_date.month())},
                                                   date::day{static_cast<unsigned int>(qt_date.day())}}};

    auto amount = snapshot.amount();
    finances::accounts::models::SnapshotManager manager{pool};
    auto r = manager.create(account.id, std::move(date), std::move(amount));
    if (!r) {
        SPDLOG_ERROR("Error adding snapshot to account");
        // TODO: Communicate error to user
        return;
    }

    emit snapshot_added(account.id);
}

template <typename TModel> struct DataDispatcher<TModel, MovementColumn, Qt::FontRole> {
    static QVariant data(const AccountRelatedModelBase&, const TModel&, MovementColumn column) {
        if ((column == MovementColumn::DATE_VALUE) || (column == MovementColumn::AMOUNT)) {
            return QVariant{QFont{"Andale Mono"}};
        }
        return QVariant{};
    }
};

template <typename TColumn> struct DataDispatcher<finances::accounts::models::Snapshot, TColumn, Qt::BackgroundRole> {
    static QVariant data(const AccountRelatedModelBase&, const finances::accounts::models::Snapshot&, TColumn column) {
        return QVariant{QColor(255, 255, 40)};
    }
};

template <>
QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementColumn, Qt::DisplayRole>::data(
    const AccountRelatedModelBase& account, const finances::accounts::models::Snapshot& item, MovementColumn column) {
    QVariant result = QVariant();

    switch (column) {
    case MovementColumn::ID:
        result = (uint64_t)item.id; // FIXME: implement the right conversion
        break;
    case MovementColumn::DATE_VALUE:
        result = QDate{int(item.date_value.year()), static_cast<int>(unsigned(item.date_value.month())),
                       static_cast<int>(unsigned(item.date_value.day()))}
                     .toString("yyyy-MM-dd");
        break;
    case MovementColumn::AMOUNT: {
        auto amount_money = finances::accounts::models::Money{item.amount, account.account.ccy};
        result = QString::fromStdString(static_cast<std::string>(amount_money));
    } break;
    // Snashot doesn't have these fields
    case MovementColumn::TRANSACTION:
    case MovementColumn::MOVE_TYPE:
    case MovementColumn::DIRECTION:
        break;
    }

    return result;
}

template <>
QVariant DataDispatcher<finances::accounts::models::Movement, MovementColumn, Qt::DisplayRole>::data(
    const AccountRelatedModelBase& account, const finances::accounts::models::Movement& item, MovementColumn column) {
    QVariant result = QVariant();
    switch (column) {
    case MovementColumn::ID:
        result = (uint64_t)item.id; // FIXME: implement the right conversion
        break;
    case MovementColumn::DATE_VALUE:
        result = QDate{int(item.date_value.year()), static_cast<int>(unsigned(item.date_value.month())),
                       static_cast<int>(unsigned(item.date_value.day()))}
                     .toString("yyyy-MM-dd");
        break;
    case MovementColumn::AMOUNT: {
        auto amount_money = finances::accounts::models::Money{item.amount, account.account.ccy};
        result = QString::fromStdString(static_cast<std::string>(amount_money));
    } break;
    case MovementColumn::TRANSACTION:
        result = item.transaction.second.c_str();
        break;
    case MovementColumn::MOVE_TYPE: {
        auto breadcrumb = account.movtype_model->get_breadcrumb(item.type.first);
        if (breadcrumb) {
            // FIXME: We are doing this in multiple places
            QString q_breadcrumb;
            for (const auto& it : breadcrumb.value().get()) {
                q_breadcrumb.append(it.c_str());
                q_breadcrumb.append(" > ");
            }
            q_breadcrumb.append(item.type.second.c_str());

            result = q_breadcrumb;
        } else {
            result = item.type.second.c_str();
        }
    } break;
    case MovementColumn::DIRECTION:
        result = QString::fromStdString(std::string(magic_enum::enum_name(item.direction)));
        break;
    }
    return result;
}

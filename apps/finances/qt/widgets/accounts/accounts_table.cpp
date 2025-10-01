#include "accounts_table.h"

#include <QCheckBox>
#include <QGroupBox>
#include <QGuiApplication>
#include <QHeaderView>
#include <QLabel>
#include <QLineEdit>
#include <QTableView>
#include <QTimer>
#include <QVBoxLayout>

#include <magic_enum/magic_enum.hpp>
#include <spdlog/spdlog.h>

#include "apps/finances/qt/widgets/accounts/non_numerable/add_snapshot.h"
#include "apps/finances/qt/widgets/accounts/numerable/add_snapshot.h"

AccountsTableWidget::AccountsTableWidget(utils::libpqxx::ConnectionPool& pool_, AccountTableModel* model_,
                                         std::optional<finances::accounts::models::AccountHolder> me, QWidget* parent)
    : QWidget(parent), pool{pool_}, model{model_} {

    // Initial values for the filters
    Qt::CheckState showClosedAccounts = Qt::Unchecked;
    Qt::CheckState showOthersAccounts = me ? Qt::Unchecked : Qt::Checked;

    // Components
    sort_filter = new AccountsTableFilterProxyModel(me, model, showClosedAccounts, showOthersAccounts, this);
    sort_filter->setSourceModel(model);
    sort_filter->setSortCaseSensitivity(Qt::CaseInsensitive);
    sort_filter->setFilterCaseSensitivity(Qt::CaseInsensitive);
    sort_filter->setFilterKeyColumn(-1); // Use all columns

    QTableView* table_view = new QTableView(this);
    table_view->setModel(sort_filter);
    table_view->setSortingEnabled(true);
    table_view->hideColumn(magic_enum::enum_integer(AccountTableModel::Column::ID));
    table_view->hideColumn(magic_enum::enum_integer(AccountTableModel::Column::IDENTIFIER));
    table_view->hideColumn(magic_enum::enum_integer(AccountTableModel::Column::OPEN));
    table_view->hideColumn(magic_enum::enum_integer(AccountTableModel::Column::CLOSE));
    table_view->verticalHeader()->hide();
    table_view->horizontalHeader()->setSectionResizeMode(QHeaderView::ResizeToContents);
    connect(table_view, &QTableView::doubleClicked, this, &AccountsTableWidget::onDoubleClicked);
    connect(table_view, &QTableView::pressed, this, &AccountsTableWidget::onPressed);

    // Filters
    // - filter by term
    QLabel* filterLabel = new QLabel(tr("Filter:"));
    QLineEdit* lineEdit = new QLineEdit;
    lineEdit->setPlaceholderText(tr("Search all fields in the table..."));
    filterLabel->setBuddy(lineEdit);
    connect(lineEdit, &QLineEdit::textChanged, sort_filter, &AccountsTableFilterProxyModel::setFilterWildcard);
    // - filter open/close accounts
    QCheckBox* showClosed = new QCheckBox(tr("Show closed"));
    showClosed->setCheckState(showClosedAccounts);
    connect(showClosed, &QCheckBox::checkStateChanged, sort_filter, &AccountsTableFilterProxyModel::showClosedAccounts);

    // Layout
    // - line with all the filters
    QHBoxLayout* filtersLine = new QHBoxLayout();
    filtersLine->addWidget(filterLabel);
    filtersLine->addWidget(lineEdit);
    filtersLine->addWidget(showClosed);
    if (me) {
        // - filter mine/other's accounts
        QCheckBox* showOthers = new QCheckBox(tr("Show others"));
        showOthers->setCheckState(showOthersAccounts);
        connect(showOthers, &QCheckBox::checkStateChanged, sort_filter,
                &AccountsTableFilterProxyModel::showOthersAccounts);

        filtersLine->addWidget(showOthers);
    }

    // - VBox for filters and table
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addLayout(filtersLine);
    mainLayout->addWidget(table_view);

    this->setLayout(mainLayout);
}

void AccountsTableWidget::onDoubleClicked(const QModelIndex& index) {
    SPDLOG_TRACE("AccountsTableWidget::onDoubleClicked(index.row={})", index.row());

    // Get the account id from the filter/sort view
    QVariant account_id_variant =
        sort_filter->data(index.siblingAtColumn(magic_enum::enum_integer(AccountTableModel::Column::ID)));
    SPDLOG_TRACE(" - account_id: {}", account_id_variant.toString().toStdString());
    finances::accounts::models::Id account_id{account_id_variant.toULongLong()};

    emit accountDoubleClicked(account_id);
}

void AccountsTableWidget::onPressed(const QModelIndex& index) {
    SPDLOG_TRACE("AccountsTableWidget::onPressed(index.row={}, index.column={})", index.row(), index.column());

    // Only if the user clicks the snapshot column
    AccountTableModel::Column column = magic_enum::enum_value<AccountTableModel::Column>(index.column());
    if (column != AccountTableModel::Column::SNAPSHOT) {
        return;
    }

    auto buttons = QGuiApplication::mouseButtons();
    if (buttons == Qt::RightButton) {
        // Get the account id from the filter/sort view
        QVariant account_id_variant =
            sort_filter->data(index.siblingAtColumn(magic_enum::enum_integer(AccountTableModel::Column::ID)));
        SPDLOG_TRACE(" - account_id: {}", account_id_variant.toString().toStdString());
        finances::accounts::models::Id account_id{account_id_variant.toULongLong()};

        // Get the account itself
        const auto& account = model->get_account(account_id);

        // Show the AddSnapshot dialog
        // FIXME: Merge AddSnapshotNumerableWidget and AddSnapshotNonNumerableWidget into a single one AddSnapshot
        // widget.
        if (account.is_numerable) {
            AddSnapshotNumerableWidget* add_snapshot = new AddSnapshotNumerableWidget(pool, account, this);
            add_snapshot->setModal(true);
            add_snapshot->setSizeGripEnabled(true);
            add_snapshot->open();
            connect(add_snapshot, &AddSnapshotNumerableWidget::new_snapshot, model, &AccountTableModel::fetch_snapshot);
        } else {
            AddSnapshotNonNumerableWidget* add_snapshot = new AddSnapshotNonNumerableWidget(pool, account, this);
            add_snapshot->setModal(true);
            add_snapshot->setSizeGripEnabled(true);
            add_snapshot->open();
            connect(add_snapshot, &AddSnapshotNonNumerableWidget::new_snapshot, model,
                    &AccountTableModel::fetch_snapshot);
        }
    }
}

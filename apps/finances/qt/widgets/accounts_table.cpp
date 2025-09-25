#include "accounts_table.h"

#include <QCheckBox>
#include <QGroupBox>
#include <QLabel>
#include <QLineEdit>
#include <QTableView>
#include <QTimer>
#include <QVBoxLayout>

AccountsTableWidget::AccountsTableWidget(utils::libpqxx::ConnectionPool& pool, QWidget* parent, Qt::WindowFlags f)
    : QWidget(parent, f) {
    model = new AccountTableModel(pool, this);

    // Initial values for these filters
    Qt::CheckState showClosedAccounts = Qt::Unchecked;
    Qt::CheckState showOthersAccounts = Qt::Unchecked;

    // Components
    sort_filter = new AccountsTableFilterProxyModel(showClosedAccounts, showOthersAccounts, this);
    sort_filter->setSourceModel(model);
    sort_filter->setSortCaseSensitivity(Qt::CaseInsensitive);
    sort_filter->setFilterCaseSensitivity(Qt::CaseInsensitive);
    sort_filter->setFilterKeyColumn(-1); // Use all columns

    QTableView* table_view = new QTableView(this);
    table_view->setModel(sort_filter);
    table_view->setSortingEnabled(true);

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
    // - filter mine/other's accounts
    QCheckBox* showOthers = new QCheckBox(tr("Show others"));
    showOthers->setCheckState(showOthersAccounts);
    connect(showOthers, &QCheckBox::checkStateChanged, sort_filter, &AccountsTableFilterProxyModel::showOthersAccounts);

    // Layout
    // - line with all the filters
    QHBoxLayout* filtersLine = new QHBoxLayout();
    filtersLine->addWidget(filterLabel);
    filtersLine->addWidget(lineEdit);
    filtersLine->addWidget(showClosed);
    filtersLine->addWidget(showOthers);

    // - VBox for filters and table
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addLayout(filtersLine);
    mainLayout->addWidget(table_view);

    this->setLayout(mainLayout);

    // We initialize the widget with all the models
    QTimer::singleShot(0, model, SLOT(fetch_all()));
}

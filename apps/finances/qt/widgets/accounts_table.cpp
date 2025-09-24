#include "accounts_table.h"

#include <QLabel>
#include <QLineEdit>
#include <QTableView>
#include <QTimer>
#include <QVBoxLayout>

AccountsTableWidget::AccountsTableWidget(utils::libpqxx::ConnectionPool& pool, QWidget* parent, Qt::WindowFlags f)
    : QWidget(parent, f) {
    model = new AccountTableModel(pool, this);

    sort_filter = new QSortFilterProxyModel(this);
    sort_filter->setSourceModel(model);
    sort_filter->setSortCaseSensitivity(Qt::CaseInsensitive);
    sort_filter->setFilterCaseSensitivity(Qt::CaseInsensitive);
    sort_filter->setFilterKeyColumn(-1); // Use all columns

    QTableView* table_view = new QTableView(this);
    table_view->setModel(sort_filter);
    table_view->setSortingEnabled(true);

    QLineEdit* lineEdit = new QLineEdit(this);
    QLabel* filterLabel = new QLabel("&Filter:", this);
    filterLabel->setBuddy(lineEdit);
    connect(lineEdit, &QLineEdit::textChanged, sort_filter, &QSortFilterProxyModel::setFilterWildcard);

    QVBoxLayout* all = new QVBoxLayout(this);
    all->addWidget(lineEdit);
    all->addWidget(table_view);

    // We initialize the widget with all the models
    QTimer::singleShot(0, model, SLOT(fetch_all()));
}

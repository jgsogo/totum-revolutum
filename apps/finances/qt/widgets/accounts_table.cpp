#include "accounts_table.h"

#include <QHBoxLayout>
#include <QTableView>
#include <QTimer>
#include <spdlog/spdlog.h>

AccountsTableWidget::AccountsTableWidget(utils::db::ConnectionPool& pool, QWidget* parent, Qt::WindowFlags f)
    : QWidget(parent, f) {
    model = new AccountTableModel(pool, this);

    sort_filter = new QSortFilterProxyModel(this);
    sort_filter->setSourceModel(model);

    QTableView* view = new QTableView(this);
    view->setModel(sort_filter);
    view->setSortingEnabled(true);

    QHBoxLayout* all = new QHBoxLayout(this);
    all->addWidget(view);

    // We initialize the widget with all the models
    QTimer::singleShot(0, model, SLOT(fetch_all()));
}

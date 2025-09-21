#include "accounts_table.h"

#include <QHBoxLayout>
#include <QTableView>
#include <spdlog/spdlog.h>

AccountsTableWidget::AccountsTableWidget(utils::db::ConnectionPool& pool, QWidget* parent, Qt::WindowFlags f)
    : QWidget(parent, f), pool{pool} {
    model = new AccountTableModel(this);
    sort_filter = new QSortFilterProxyModel(this);
    QTableView* view = new QTableView(this);

    sort_filter->setSourceModel(model);
    view->setModel(sort_filter);
    view->setSortingEnabled(true);

    QHBoxLayout* all = new QHBoxLayout(this);
    all->addWidget(view);

    QTimer::singleShot(0, this, SLOT(refresh_all_accounts()));
}

void AccountsTableWidget::refresh_all_accounts() {
    SPDLOG_DEBUG("AccountsTableWidget::refresh_all_accounts");
    finances::accounts::models::AccountManager manager{pool};
    auto all_accounts = manager.all();
    if (!all_accounts) {
        SPDLOG_ERROR("Error refreshing accounts");
        // TODO: Communicate error to user
    }
    model->set_accounts(std::move(all_accounts.value()));
}

void AccountsTableWidget::refresh_account(finances::accounts::models::Id id) {
    SPDLOG_DEBUG("AccountsTableWidget::refresh_account(id={})", id);
}

#include "main_tab.h"

#include <spdlog/spdlog.h>

#include "account_detail.h"
#include "accounts_table.h"

MainTabWidget::MainTabWidget(utils::libpqxx::ConnectionPool& pool, QWidget* parent) : QTabWidget(parent), pool{pool} {

    AccountsTableWidget* accounts_table_widget = new AccountsTableWidget(pool);
    _all_accounts_idx = this->addTab(accounts_table_widget, "All");
    connect(accounts_table_widget, &AccountsTableWidget::accountDoubleClicked, this, &MainTabWidget::addTabAccount);
}

void MainTabWidget::addTabAccount(finances::accounts::models::Id account_id) {
    SPDLOG_DEBUG("MainTabWidget::addTabAccount(account_id={})", account_id);

    // TODO: Check if this tab is already available, and show it

    // Create a new tab for this account
    AccountDetailWidget* account_widget = new AccountDetailWidget(pool, this);
    this->addTab(account_widget, "Account: name");
}

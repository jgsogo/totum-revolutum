#include "main_tab.h"

#include <spdlog/spdlog.h>

#include "apps/finances/qt/models/account_related.h"

#include "account_detail.h"
#include "accounts_table.h"

MainTabWidget::MainTabWidget(utils::libpqxx::ConnectionPool& pool_, AccountTableModel* model_, QWidget* parent)
    : QTabWidget(parent), pool{pool_}, model{model_} {
    this->setTabsClosable(true);

    // Add the tab with the accounts table
    AccountsTableWidget* accounts_table_widget = new AccountsTableWidget(model);
    _all_accounts_idx = this->addTab(accounts_table_widget, "All");
    connect(accounts_table_widget, &AccountsTableWidget::accountDoubleClicked, this, &MainTabWidget::addTabAccount);

    // Remove the close buttom from this ALL tab
    auto leftTabButton = this->tabBar()->tabButton(_all_accounts_idx, QTabBar::LeftSide);
    if (leftTabButton)
        leftTabButton->resize(0, 0);
    auto rightTabButton = this->tabBar()->tabButton(_all_accounts_idx, QTabBar::RightSide);
    if (rightTabButton)
        rightTabButton->resize(0, 0);

    // connect other signals
    connect(this, &MainTabWidget::tabCloseRequested, this, &MainTabWidget::closeTab);
}

void MainTabWidget::addTabAccount(finances::accounts::models::Id account_id) {
    SPDLOG_DEBUG("MainTabWidget::addTabAccount(account_id={})", account_id);

    // If this tab is already available, show it
    auto found = _accounts_tabs.find(account_id);
    if (found != _accounts_tabs.end()) {
        SPDLOG_TRACE(" - widget for account already available. Just switch to it");
        this->setCurrentIndex(found->second);
        return;
    }

    // Get the data for this account
    try {
        const auto& account = model->get_account(account_id);
        AccountSnapshotsModel* snapshots_model = new AccountSnapshotsModel(pool, account, this);
        AccountMovementsModel* movements_model = new AccountMovementsModel(pool, account, this);

        AccountDetailWidget* account_widget = new AccountDetailWidget(account, snapshots_model, movements_model, this);
        auto idx = this->addTab(account_widget,
                                QString("%1 - %2").arg(account.custodian.second.c_str()).arg(account.name.c_str()));
        _accounts_tabs.insert(std::make_pair(account_id, idx));

        // and make it active
        this->setCurrentIndex(idx);
    } catch (std::runtime_error) {
        SPDLOG_ERROR("Account {} is not in the model!", account_id);
        return;
    }
}

void MainTabWidget::closeTab(int index) {
    SPDLOG_DEBUG("MainTabWidget::closeTab(index={})", index);
    assert(index != _all_accounts_idx && "Requested to close ALL tab. Not expected!");

    // Remove from the vector
    std::erase_if(_accounts_tabs, [index](const auto& item) { return item.second == index; });

    // Remove the tab
    this->removeTab(index);
}

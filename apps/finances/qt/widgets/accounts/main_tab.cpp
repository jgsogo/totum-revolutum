#include "main_tab.h"

#include <spdlog/spdlog.h>

#include "non_numerable/account_detail.h"
#include "numerable/account_detail.h"

#include "accounts_table.h"

MainTabWidget::MainTabWidget(utils::libpqxx::ConnectionPool& pool_, AccountTableModel* model_,
                             const MovementTypeTableModel* movtype_model_, QWidget* parent)
    : QTabWidget(parent), pool{pool_}, model{model_}, movtype_model{movtype_model_} {
    this->setTabsClosable(true);

    // Add the tab with the accounts table
    AccountsTableWidget* accounts_table_widget = new AccountsTableWidget(pool, model);
    _all_accounts_idx = this->addTab(accounts_table_widget, "All");
    connect(accounts_table_widget, &AccountsTableWidget::accountDoubleClicked, this, &MainTabWidget::addTabAccount);

    // Remove the close buttom from this ALL tab
    auto leftTabButton = this->tabBar()->tabButton(_all_accounts_idx, QTabBar::LeftSide);
    if (leftTabButton)
        leftTabButton->resize(0, 0);
    auto rightTabButton = this->tabBar()->tabButton(_all_accounts_idx, QTabBar::RightSide);
    if (rightTabButton)
        rightTabButton->resize(0, 0);

    connect(this, &MainTabWidget::tabCloseRequested, this, &MainTabWidget::closeMyTab);
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

        // create the widget
        AccountDetailWidget* account_widget = nullptr;
        if (!account.is_numerable) {
            account_widget = new AccountNonNumerableDetailWidget(pool, account, movtype_model, this);
        } else {
            account_widget = new AccountNumerableDetailWidget(pool, account, movtype_model, this);
        }
        connect(account_widget, &AccountDetailWidget::snapshot_added, [this](auto id) { emit account_changed(id); });
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

void MainTabWidget::closeMyTab(int index) {
    SPDLOG_DEBUG("MainTabWidget::closeMyTab(index={})", index);
    assert(index != _all_accounts_idx && "Requested to close ALL tab. Not expected!");
    this->removeTab(index);
}

void MainTabWidget::tabRemoved(int index) {
    SPDLOG_DEBUG("MainTabWidget::tabRemoved(index={})", index);
    std::erase_if(_accounts_tabs, [index](const auto& item) { return item.second == index; });
}

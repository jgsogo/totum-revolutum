#include "main_tab.h"

#include <spdlog/spdlog.h>

#include "accounts_table.h"

MainTabWidget::MainTabWidget(utils::libpqxx::ConnectionPool& pool_,
                             std::optional<finances::accounts::models::AccountHolder> me,
                             AccountsTableModel<AccountColumns>& accounts_,
                             MovementTypesTableModel<HierarchyTreeColumns>& movtypes_, QWidget* parent)
    : QTabWidget(parent), pool{pool_}, accounts{accounts_}, movtypes{movtypes_} {
    this->setTabsClosable(true);

    // Add the tab with the accounts table
    AccountsTableWidget* accounts_table_widget = new AccountsTableWidget(pool, accounts_, me);
    _all_accounts_idx = this->addTab(accounts_table_widget, "All");
    assert(_all_accounts_idx == 0 && "All accounts tab index should be zero");
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

void MainTabWidget::addTabAccount(utils::db::Id account_id) {
    SPDLOG_DEBUG("MainTabWidget::addTabAccount(account_id={})", account_id);

    // If this tab is already available, show it
    auto found = std::find_if(_accounts_tabs.begin(), _accounts_tabs.end(),
                              [&account_id](AccountDetailWidget* w) { return w->get_account().id == account_id; });
    if (found != _accounts_tabs.end()) {
        SPDLOG_TRACE(" - widget for account already available. Just switch to it. Position {}",
                     std::distance(_accounts_tabs.begin(), found) + 1);
        this->setCurrentIndex(std::distance(_accounts_tabs.begin(), found) + 1);
        return;
    }

    // Get the data for this account
    try {
        const auto& account_expected = accounts.get(account_id);
        if (!account_expected) {
            SPDLOG_ERROR("AccountModel with id {} not found in the MainTab", account_id);
            return;
        }
        const AccountModel& account = account_expected.value();

        // create the widget
        AccountDetailWidget* account_widget = new AccountDetailWidget{pool, accounts, movtypes, account, this};
        connect(account_widget, &AccountDetailWidget::snapshot_added, [this](auto id) { emit account_changed(id); });
        auto idx = this->addTab(
            account_widget,
            QString("%1 - %2").arg(account.account.custodian.second.c_str()).arg(account.account.name.c_str()));
        SPDLOG_TRACE(" - new widget added as index {} (_accounts_tabs.size() == {})", idx, _accounts_tabs.size());
        _accounts_tabs.push_back(account_widget);
        assert(idx == _accounts_tabs.size() && "Account detail widgets are added at the end of the vector");

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
    _accounts_tabs.erase(_accounts_tabs.begin() + index -
                         1); // We substract 1, the "All accounts" is not added to the _account_tabs vector
}

void MainTabWidget::on_account_changed(utils::db::Id account_id) {
    SPDLOG_DEBUG("MainTabWidget::on_account_changed(account_id={})", account_id);

    auto found = std::find_if(_accounts_tabs.begin(), _accounts_tabs.end(),
                              [&account_id](AccountDetailWidget* w) { return w->get_account().id == account_id; });
    if (found != _accounts_tabs.end()) {
        (*found)->refresh(account_id);
    }
}

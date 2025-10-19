#pragma once

#include <QTabWidget>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "apps/finances/qt/table_models/accounts.h"
#include "apps/finances/qt/tables/accounts.h"

class MainTabWidget : public QTabWidget {
    Q_OBJECT

  public:
    MainTabWidget(utils::libpqxx::ConnectionPool& pool, std::optional<finances::accounts::models::AccountHolder> me,
                  AccountsTableModel<AccountColumns>* model, QWidget* parent = nullptr);
    void tabRemoved(int index) override;

  private slots:
    void addTabAccount(utils::db::Id account_id);
    void closeMyTab(int);

  signals:
    void account_changed(utils::db::Id account_id);

  private:
    int _all_accounts_idx;
    std::unordered_map<utils::db::Id, int> _accounts_tabs;

    utils::libpqxx::ConnectionPool& pool;
    AccountsTableModel<AccountColumns>* model;
};

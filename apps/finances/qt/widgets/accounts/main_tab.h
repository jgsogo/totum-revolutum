#pragma once

#include <QTabWidget>

#include "libraries/finances/accounts/cpp/models/types/id.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "apps/finances/qt/models/accounts_table.h"
#include "apps/finances/qt/models/movement_type.h"

class MainTabWidget : public QTabWidget {
    Q_OBJECT

  public:
    MainTabWidget(utils::libpqxx::ConnectionPool& pool, std::optional<finances::accounts::models::AccountHolder> me,
                  AccountTableModel* model, const MovementTypeTableModel* movtype_model, QWidget* parent = nullptr);
    void tabRemoved(int index) override;

  private slots:
    void addTabAccount(finances::accounts::models::Id account_id);
    void closeMyTab(int);

  signals:
    void account_changed(finances::accounts::models::Id account_id);

  private:
    int _all_accounts_idx;
    std::unordered_map<finances::accounts::models::Id, int> _accounts_tabs;

    utils::libpqxx::ConnectionPool& pool;
    AccountTableModel* model;
    const MovementTypeTableModel* movtype_model;
};

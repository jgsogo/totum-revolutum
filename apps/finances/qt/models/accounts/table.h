#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "columns.h"
#include "model.h"

class AccountsTable final : public utils::qt::models::TableModel<AccountModel, AccountColumns> {
  public:
    AccountsTable(utils::libpqxx::ConnectionPool& pool, QObject* parent = nullptr);

  protected:
    void _refresh_all() override;
    void _refresh_one(const utils::db::ModelData<AccountModel>::Id& id, int row) override final;
};

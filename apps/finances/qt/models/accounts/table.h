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

namespace utils::qt::models {

    template <>
    QVariant
    DataDispatcher<AccountModel, AccountColumns, Qt::DisplayRole>::data(const TableModel<AccountModel, AccountColumns>&,
                                                                        const AccountModel&, AccountColumns);

    template <>
    QVariant
    DataDispatcher<AccountModel, AccountColumns, Qt::FontRole>::data(const TableModel<AccountModel, AccountColumns>&,
                                                                     const AccountModel&, AccountColumns);

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::ForegroundRole>::data(
        const TableModel<AccountModel, AccountColumns>&, const AccountModel&, AccountColumns);

    template <>
    QVariant DataDispatcher<AccountModel, AccountColumns, Qt::TextAlignmentRole>::data(
        const TableModel<AccountModel, AccountColumns>&, const AccountModel&, AccountColumns);
} // namespace utils::qt::models

#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "columns.h"
#include "model.h"

class AccountsTable final : public utils::qt::models::TableModel<AccountModel, AccountColumns> {
  public:
    AccountsTable(utils::libpqxx::ConnectionPool& pool, QObject* parent = nullptr);
};

namespace utils::qt::models {

    template <>
    QVariant
    DataDispatcher<AccountModel, AccountColumns, Qt::DisplayRole>::data(const TableModel<AccountModel, AccountColumns>&,
                                                                        const AccountModel&, AccountColumns);

} // namespace utils::qt::models

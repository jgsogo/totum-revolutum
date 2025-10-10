#pragma once

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/utils/cpp/qt/models/generic_table_model.h"

enum class AccountColumns {
    ID = 0,
    CUSTODIAN = 1,
    NAME = 2,
    IDENTIFIER = 3,
    TYPE = 4,
    SNAPSHOT = 5,
    OPEN = 6,
    CLOSE = 7,
    HOLDERS = 8,
};

class AccountsTable final : public utils::qt::models::TableModel<finances::accounts::models::Account, AccountColumns> {
  public:
    AccountsTable(utils::libpqxx::ConnectionPool& pool, QObject* parent = nullptr);
};

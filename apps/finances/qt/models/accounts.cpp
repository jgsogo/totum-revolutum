#include "accounts.h"

AccountsTable::AccountsTable(utils::libpqxx::ConnectionPool& pool, QObject* parent)
    : utils::qt::models::TableModel<finances::accounts::models::Account, AccountColumns>{pool, parent} {}
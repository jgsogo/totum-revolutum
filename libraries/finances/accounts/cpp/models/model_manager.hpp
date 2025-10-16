#pragma once

#include <vector>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"
#include "tl/expected.hpp"

#include "errors.h"

namespace finances::accounts::models {

    static constexpr std::string_view ACCOUNT_TABLE = "finances_accounts_account";
    // static constexpr std::string_view ACCOUNT_TYPE_TABLE = "finances_accounts_accounttype";
    static constexpr utils::StringLiteral ACCOUNT_TYPE_TABLE{"finances_accounts_accounttype"};
    static constexpr std::string_view CUSTODIAN_TABLE = "finances_accounts_custodian";
    static constexpr std::string_view SNAPSHOT_TABLE = "finances_accounts_snapshot";
    static constexpr std::string_view MOVEMENT_TABLE = "finances_accounts_movement";
    static constexpr utils::StringLiteral MOVEMENTTYPE_TABLE{"finances_accounts_movementtype"};
    static constexpr std::string_view TRANSACTION_TABLE = "finances_accounts_transaction";
    static constexpr std::string_view ACCOUNT_HOLDER_TABLE = "finances_accounts_accountholder";
    static constexpr std::string_view ACCOUNT_HOLDER_ROLE_TABLE = "finances_accounts_accountholderrole";

} // namespace finances::accounts::models

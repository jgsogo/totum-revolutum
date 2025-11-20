#pragma once

#include "libraries/utils/cpp/string_literal.hpp"

namespace finances::accounts::models {

    static constexpr std::string_view ACCOUNT_TABLE = "finances_accounts_account";
    // static constexpr std::string_view ACCOUNT_TYPE_TABLE = "finances_accounts_accounttype";
    static constexpr utils::StringLiteral ACCOUNT_TYPE_TABLE{"finances_accounts_accounttype"};
    static constexpr std::string_view CUSTODIAN_TABLE = "finances_accounts_custodian";
    static constexpr std::string_view SNAPSHOT_TABLE = "finances_accounts_snapshot";
    static constexpr std::string_view MOVEMENT_TABLE = "finances_accounts_movement";
    static constexpr utils::StringLiteral MOVEMENTTYPE_TABLE{"finances_accounts_movementtype"};
    static constexpr std::string_view TRANSACTION_TABLE = "finances_accounts_transaction";
    static constexpr std::string_view TRANSACTION_GROUP_TABLE = "finances_accounts_transactiongroup";
    static constexpr std::string_view ACCOUNT_HOLDER_TABLE = "finances_accounts_accountholder";
    static constexpr std::string_view ACCOUNT_HOLDER_ROLE_TABLE = "finances_accounts_accountholderrole";
    static constexpr std::string_view FX_TABLE = "finances_accounts_fx";

} // namespace finances::accounts::models

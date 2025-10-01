#pragma once

#include <vector>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"
#include "tl/expected.hpp"

#include "errors.h"
#include "types/id.h"

namespace finances::accounts::models {

    static constexpr std::string_view ACCOUNT_TABLE = "finances_accounts_account";
    static constexpr std::string_view ACCOUNT_TYPE_TABLE = "finances_accounts_accounttype";
    static constexpr std::string_view CUSTODIAN_TABLE = "finances_accounts_custodian";
    static constexpr std::string_view SNAPSHOT_TABLE = "finances_accounts_snapshot";
    static constexpr std::string_view MOVEMENT_TABLE = "finances_accounts_movement";
    static constexpr std::string_view MOVEMENTTYPE_TABLE = "finances_accounts_movementtype";
    static constexpr std::string_view TRANSACTION_TABLE = "finances_accounts_transaction";
    static constexpr std::string_view ACCOUNT_HOLDER_TABLE = "finances_accounts_accountholder";
    static constexpr std::string_view ACCOUNT_HOLDER_ROLE_TABLE = "finances_accounts_accountholderrole";

    template <typename TModel> class ModelManager {
      public:
        ModelManager(utils::libpqxx::ConnectionPool& pool) : pool{pool} {}

        tl::expected<std::vector<TModel>, Error> all() { return tl::unexpected(Error::NotImplemented); };
        tl::expected<TModel, Error> get(Id id) { return tl::unexpected(Error::NotImplemented); };

      protected:
        utils::libpqxx::ConnectionPool& pool;
    };

} // namespace finances::accounts::models

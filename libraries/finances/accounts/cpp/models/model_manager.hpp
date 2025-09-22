#pragma once

#include <vector>

#include "libraries/utils/cpp/db/connection_pool.h"
#include "tl/expected.hpp"

#include "errors.h"

namespace finances::accounts::models {

    static constexpr std::string_view ACCOUNT_TABLE = "finances_accounts_account";
    static constexpr std::string_view ACCOUNT_TYPE_TABLE = "finances_accounts_accounttype";
    static constexpr std::string_view CUSTODIAN_TABLE = "finances_accounts_custodian";
    static constexpr std::string_view SNAPSHOT_TABLE = "finances_accounts_snapshot";

    template <typename TModel> class ModelManager {
      public:
        ModelManager(utils::db::ConnectionPool& pool) : pool{pool} {}

        tl::expected<std::vector<TModel>, Error> all();

      protected:
        utils::db::ConnectionPool& pool;
    };

} // namespace finances::accounts::models

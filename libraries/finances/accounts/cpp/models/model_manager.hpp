#pragma once

#include <vector>

#include "libraries/utils/cpp/db/connection_pool.h"
#include "tl/expected.hpp"

#include "errors.h"

namespace finances::accounts::models {

    static constexpr std::string_view ACCOUNT_TABLE = "finances_accounts_account";

    template <typename TModel> class ModelManager {
      public:
        ModelManager(utils::db::ConnectionPool& pool) : pool{pool} {}

        tl::expected<std::vector<TModel>, Error> all();

      private:
        utils::db::ConnectionPool& pool;
    };

} // namespace finances::accounts::models

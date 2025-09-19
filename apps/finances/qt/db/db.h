#pragma once

#include <vector>

#include "libraries/finances/accounts/cpp/models/account.h"

namespace db {

    class DB {
      public:
        virtual ~DB() = 0;

        virtual std::vector<finances::accounts::models::Account> get_accounts() = 0;
    };
} // namespace db

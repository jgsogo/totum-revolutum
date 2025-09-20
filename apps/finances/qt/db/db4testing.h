#pragma once

#include "db.h"

namespace test::db {

    class Db4Testing final : public ::db::DB {
        Db4Testing();
        virtual ~Db4Testing();

        std::vector<finances::accounts::models::Account> get_accounts() override;
    };
} // namespace test::db

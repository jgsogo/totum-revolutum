#include "db4testing.h"

#include <string_view>

namespace test::db {

    namespace {
        using namespace finances::accounts::models;

        // static std::vector<Account> accounts = {
        //     AccountType{
        //         .id = 0u,
        //         .name = "acc_type1",
        //         .description = "description1",
        //         .identifier = "id1",
        //         .ccy = EUR,
        //         .type_id = 0u,
        //         .custodian_id = 0u,
        //         .is_numerable = false,
        //     },
        // };

        // Id id;
        // std::string name;
        // std::optional<std::string> description;
        // bool is_abstract;
        // std::optional<std::string> unique_name;

        static std::vector<Account> accounts = {
            Account{
                .id = 0u,
                .name = "account1",
                .description = "description1",
                .identifier = "id1",
                .ccy = EUR,
                .type_id = 0u,
                .custodian_id = 0u,
                .is_numerable = false,
            },
        };
    } // namespace

    Db4Testing::Db4Testing() {}
    Db4Testing::~Db4Testing() {}

    std::vector<finances::accounts::models::Account> Db4Testing::get_accounts() { return accounts; }
}; // namespace test::db

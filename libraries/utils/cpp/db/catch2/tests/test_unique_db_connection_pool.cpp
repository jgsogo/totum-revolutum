#include <catch2/catch_test_macros.hpp>

#include "libraries/utils/cpp/db/catch2/unique_db_connection_pool.hpp"

TEST_CASE_PERSISTENT_FIXTURE(UniqueDBConnectionPool, "Test connection pool") {

    SECTION("Test pool can provide connections") {
        pool.with_conn<void>([](pqxx::connection& conn) {
            pqxx::work tx(conn);
            std::vector<std::string> ret;
            for (auto [id] : tx.query<std::string>("SELECT id FROM users;")) {
                ret.emplace_back(id);
            }
            REQUIRE(ret.size() == 3);
        });
    }
}

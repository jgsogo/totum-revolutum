#include <catch2/catch_test_macros.hpp>

#include "apps/board_games/engine/db/tests/fixtures.hpp"

TEST_CASE_PERSISTENT_FIXTURE(DBConnectionPool, "Test connection pool") {

    SECTION("Test pool can provide connections") {
        // FIXME: We test the connection pool by retrieving the rooms, but we should test
        //        the number of connection in the pool that they are released properly,...
        pool.with_conn<void>([](pqxx::connection& conn) {
            pqxx::work tx(conn);
            std::vector<std::string> ret;
            for (auto [id] : tx.query<std::string>("SELECT id FROM board_games_core_room;")) {
                ret.emplace_back(id);
            }
            REQUIRE(ret.size() == 3);
        });
    }
}

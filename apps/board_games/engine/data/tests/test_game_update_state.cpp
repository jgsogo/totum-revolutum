#include <catch2/catch_test_macros.hpp>

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/db/tests/fixtures.hpp"

TEST_CASE_PERSISTENT_FIXTURE(UniqueDBConnectionPool, "Test game: update state") {

    SECTION("Update game state") {
        std::int64_t game_id = 1;
        pool.with_conn<void>([&game_id](pqxx::connection& conn) {
            auto r = data::update_game_state(conn, game_id, data::GameState::PLAYING, "state_data");
            REQUIRE(r.has_value());
        });
    }
}

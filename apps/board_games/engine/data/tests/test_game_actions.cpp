#include <catch2/catch_test_macros.hpp>

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/db/tests/fixtures.hpp"

TEST_CASE_PERSISTENT_FIXTURE(UniqueDBConnectionPool, "Test game: game_action / event_log") {

    SECTION("Store game action and associated eventlog") {
        std::int64_t game_id = 1;
        data::ParticipantUUID participant{"00000000-0000-0000-0000-000000000001"};
        pool.with_conn<void>([&game_id, &participant](pqxx::connection& conn) {
            data::GameActionPayload action_payload{"payload"};
            auto r = data::store_action(conn, game_id, participant, "action_type", action_payload, false);
            REQUIRE(r.has_value());
            std::int64_t action_id = r.value();

            data::EventLogPayload event_payload{"payload"};
            auto r2 = data::store_eventlog(conn, game_id, "event_type", event_payload, action_id);
            REQUIRE(r2.has_value());
        });
    }
}

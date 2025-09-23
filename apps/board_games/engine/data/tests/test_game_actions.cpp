#include <catch2/catch_test_macros.hpp>
#include <google/protobuf/empty.pb.h>

#include "libraries/utils/cpp/catch2/unique_db_connection_pool.hpp"

#include "apps/board_games/engine/data/game.h"

using namespace utils::libpqxx::testing;

TEST_CASE_PERSISTENT_FIXTURE(UniqueDBConnectionPool, "Test game: game_action / event_log") {

    SECTION("Store game action and associated eventlog") {
        std::int64_t game_id = 1;
        data::ParticipantUUID participant{"00000000-0000-0000-0000-000000000001"};
        pool.with_conn<void>([&game_id, &participant](pqxx::connection& conn) {
            auto action_payload = data::GameActionPayload::from_proto(google::protobuf::Empty{});
            auto r = data::store_action(conn, game_id, participant, "action_type", action_payload.value(), false);
            REQUIRE(r.has_value());
            std::int64_t action_id = r.value();

            auto event_payload = data::EventLogPayload::from_proto(google::protobuf::Empty{});
            auto r2 = data::store_eventlog(conn, game_id, "event_type", event_payload.value(), action_id);
            REQUIRE(r2.has_value());
        });
    }
}

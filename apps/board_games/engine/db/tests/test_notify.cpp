#include <catch2/catch_test_macros.hpp>

#include "apps/board_games/engine/db/notify.h"
#include "fixtures.hpp"

TEST_CASE_PERSISTENT_FIXTURE(UniqueDBConnectionPool, "Test notify") {

    SECTION("Test notify without payload") {
        const std::string channel = "channel";
        bool notification_arrived = false;

        pool.with_conn<void>([&](pqxx::connection& conn) {
            conn.listen(channel, [&notification_arrived](pqxx::notification n) { notification_arrived = true; });
            conn.get_notifs();

            REQUIRE(!notification_arrived);

            db::notify(conn, channel);

            int received{conn.await_notification(3)};
            REQUIRE(received == 1);
            REQUIRE(notification_arrived);
        });
    }

    SECTION("Test notify with payload") {
        const std::string channel = "channel";
        const std::string payload = "the payload";
        std::string recv_payload;

        pool.with_conn<void>([&](pqxx::connection& conn) {
            conn.listen(channel, [&recv_payload](pqxx::notification n) { recv_payload = n.payload; });
            conn.get_notifs();

            db::notify(conn, channel, payload);

            int received{conn.await_notification(3)};
            REQUIRE(received == 1);
            REQUIRE(payload == recv_payload);
        });
    }
}

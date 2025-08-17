#include <catch2/catch_test_macros.hpp>
#include <spdlog/spdlog.h>

#include "libraries/cpp/spdlog/utils.hpp"

#include "apps/board_games/engine/data/room.h"
#include "apps/board_games/engine/db/tests/fixtures.hpp"

TEST_CASE_PERSISTENT_FIXTURE(DBConnectionPool, "Test room associated methods") {

    SECTION("Get playing rooms") {
        pool.with_conn<void>([](pqxx::connection& conn) {
            auto rooms = data::get_playing_rooms(conn);
            REQUIRE(rooms.has_value());
            REQUIRE(rooms.value().size() == 3);
        });
    }

    SECTION("Insert new room") {
        pool.with_conn<void>([](pqxx::connection& conn) {
            data::RoomUUID uuid{"c66cad9a-a72a-4612-9c93-15c52b08bc5e"};
            auto r = data::insert_new_room(conn, uuid, "room_name");
            REQUIRE(r.has_value());

            // ...and now we have 4 rooms
            auto rooms = data::get_playing_rooms(conn);
            REQUIRE(rooms.value().size() == 4);
        });
    }

    SECTION("it fails to insert a duplicated room") {
        pool.with_conn<void>([](pqxx::connection& conn) {
            data::RoomUUID uuid{"11111111-1111-1111-1111-111111111111"};
            spdlog::utils::with_level<void>(spdlog::level::off, [&conn, &uuid]() {
                auto r = data::insert_new_room(conn, uuid, "room_name");
                REQUIRE(!r.has_value());
            });
        });
    }
}

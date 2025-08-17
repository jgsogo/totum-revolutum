#include <catch2/catch_test_macros.hpp>

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/data/room.h"
#include "apps/board_games/engine/db/tests/fixtures.hpp"

TEST_CASE_PERSISTENT_FIXTURE(DBConnectionPool, "Test game associated methods") {

    const data::RoomUUID room_uuid{"01673cff-660e-4241-9f72-ae0302da6712"};

    SECTION("Remove game from non-existing room") {
        pool.with_conn<void>([&room_uuid](pqxx::connection& conn) {
            auto r = data::remove_game(conn, room_uuid);
            REQUIRE(r.has_value());

            r = data::remove_game(conn, room_uuid);
            REQUIRE(r.has_value());
        });
    }

    SECTION("Remove game from room without game associated") {
        pool.with_conn<void>([&room_uuid](pqxx::connection& conn) {
            auto r = data::insert_new_room(conn, room_uuid, "a new room");
            REQUIRE(r.has_value());

            r = data::remove_game(conn, room_uuid);
            REQUIRE(r.has_value());

            r = data::remove_game(conn, room_uuid);
            REQUIRE(r.has_value());
        });
    }

    SECTION("Start a game in a room (no previous game)") {
        pool.with_conn<void>([&room_uuid](pqxx::connection& conn) {
            data::GameType game_type{"tic_tac_toe"};
            auto r = data::start_game(conn, room_uuid, game_type);
            REQUIRE(r.has_value());

            // ...but inserting again fails
            r = data::start_game(conn, room_uuid, game_type);
            REQUIRE(!r.has_value());
        });
    }

    SECTION("it fails if the game doesn't exists") {
        pool.with_conn<void>([&room_uuid](pqxx::connection& conn) {
            data::GameType game_type{"invalid-game"};
            auto r = data::start_game(conn, room_uuid, game_type);
            REQUIRE(!r.has_value());
        });
    }
}

#include <catch2/catch_test_macros.hpp>
#include <google/protobuf/empty.pb.h>

#include "libraries/cpp/spdlog/utils.hpp"

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/data/room.h"
#include "apps/board_games/engine/db/tests/fixtures.hpp"

TEST_CASE_PERSISTENT_FIXTURE(UniqueDBConnectionPool, "Test game associated methods") {

    SECTION("Remove game from non-existing room") {
        pool.with_conn<void>([](pqxx::connection& conn) {
            const data::RoomUUID room_uuid{"01673cff-660e-4241-9f72-ae0302da6712"};
            auto r = data::remove_game(conn, room_uuid);
            REQUIRE(r.has_value());

            r = data::remove_game(conn, room_uuid);
            REQUIRE(r.has_value());
        });
    }

    data::RoomUUID room_uuid{"11111111-1111-1111-1111-111111111111"};
    SECTION("Remove game from room") {
        pool.with_conn<void>([&room_uuid](pqxx::connection& conn) {
            auto r = data::remove_game(conn, room_uuid);
            REQUIRE(r.has_value());

            r = data::remove_game(conn, room_uuid);
            REQUIRE(r.has_value());
        });
    }

    SECTION("Start a game in a room (no previous game)") {
        pool.with_conn<void>([&room_uuid](pqxx::connection& conn) {
            data::GameType game_type{"tic_tac_toe"};
            auto game_state_payload = data::GameStatePayload::from_proto(google::protobuf::Empty{});
            auto r = data::start_game(conn, room_uuid, game_type, game_state_payload.value());
            REQUIRE(r.has_value());

            // ...but inserting again fails
            spdlog::utils::with_level<void>(spdlog::level::off, [&conn, &room_uuid, &game_type]() {
                auto game_state_payload = data::GameStatePayload::from_proto(google::protobuf::Empty{});
                auto r = data::start_game(conn, room_uuid, game_type, game_state_payload.value());
                REQUIRE(!r.has_value());
            });
        });
    }

    SECTION("it fails if the game doesn't exists") {
        pool.with_conn<void>([&room_uuid](pqxx::connection& conn) {
            spdlog::utils::with_level<void>(spdlog::level::off, [&conn, &room_uuid]() {
                data::GameType game_type{"invalid-game"};
                auto game_state_payload = data::GameStatePayload::from_proto(google::protobuf::Empty{});
                auto r = data::start_game(conn, room_uuid, game_type, game_state_payload.value());
                REQUIRE(!r.has_value());
            });
        });
    }
}

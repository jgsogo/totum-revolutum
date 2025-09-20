#include <catch2/catch_test_macros.hpp>
#include <google/protobuf/empty.pb.h>
#include <spdlog/spdlog.h>

#include "libraries/cpp/spdlog/utils.hpp"
#include "libraries/utils/cpp/db/catch2/unique_db_connection_pool.hpp"

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/data/room.h"

using namespace utils::db::testing;

TEST_CASE_PERSISTENT_FIXTURE(UniqueDBConnectionPool, "Test room associated methods") {

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

    SECTION("Add participants to a room") {
        pool.with_conn<void>([](pqxx::connection& conn) {
            data::RoomUUID room_uuid{"01d1e662-3591-4d6f-aa05-d92dfacaa287"};
            data::ParticipantUUID player{"6d603593-0aaf-4c37-9b7b-fe548def909c"};
            data::ParticipantUUID spectator{"4c79d891-084e-49ad-b8b5-cb5618cde04a"};

            REQUIRE(data::insert_new_room(conn, room_uuid, "another_name").has_value());

            // Inserting a player fails if there is no game yet
            auto r = data::add_participant(conn, room_uuid, player, data::ParticipantRole::PLAYER);
            REQUIRE(!r.has_value());

            // ...but I can insert a spectator
            r = data::add_participant(conn, room_uuid, spectator, data::ParticipantRole::SPECTATOR);

            // After adding a game, I can insert the player
            auto game_state_payload = data::GameStatePayload::from_proto(google::protobuf::Empty{});
            REQUIRE(data::start_game(conn, room_uuid, data::GameType{"tic_tac_toe"}, game_state_payload.value())
                        .has_value());
            REQUIRE(data::add_participant(conn, room_uuid, player, data::ParticipantRole::PLAYER).has_value());

            // TODO: Retrieve the participants in the room and run some asserts
        });
    }

    SECTION("Notify room_update") {
        pool.with_conn<void>([](pqxx::connection& conn) {
            std::string payload;
            conn.listen("room_update", [&payload](pqxx::notification n) { payload = n.payload; });
            conn.get_notifs();

            {
                data::RoomUUID room{"08b4bef1-3663-4d00-9869-b6e850ac5525"};
                auto r = data::notify_room_update(conn, room);
                REQUIRE(r.has_value());

                int received{conn.await_notification(3)};
                REQUIRE(received == 1);
                REQUIRE(payload == std::string{room});
            }

            {
                data::RoomUUID room{"6313687b-cbd4-485f-aaa1-53d8971f5259"};
                auto r = data::notify_room_update(conn, room);
                REQUIRE(r.has_value());

                int received{conn.await_notification(3)};
                REQUIRE(received == 1);
                REQUIRE(payload == std::string{room});
            }
        });
    }
}

#include <catch2/catch_test_macros.hpp>

#include "libraries/utils/cpp/catch2/unique_db_connection_pool.hpp"

#include "apps/board_games/engine/data/constants.hpp"
#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/data/room.h"
#include "apps/board_games/games/tic_tac_toe/engine/tic_tac_toe.h"

using namespace utils::libpqxx::testing;

std::vector<board_games::tic_tac_toe::Player> to_v_players(const google::protobuf::RepeatedField<int>& board_status) {
    std::vector<board_games::tic_tac_toe::Player> v;
    v.reserve(board_status.size());

    for (int raw : board_status) {
        v.push_back(static_cast<board_games::tic_tac_toe::Player>(raw));
    }

    return v;
}

TEST_CASE_PERSISTENT_FIXTURE(UniqueDBConnectionPool, "Test DB interactions") {
    data::RoomUUID room_uuid{uuids::to_string(uuids::uuid_system_generator{}())};
    board_games::tic_tac_toe::TicTacToePlugin ttt;

    SECTION("Test board saved and recovered") {
        pool.with_conn<void>([&room_uuid, &ttt](pqxx::connection& conn) {
            REQUIRE(data::insert_new_room(conn, room_uuid, "a new room"));

            // Store new board in the database
            auto board = ttt.new_board();
            auto r = data::start_game(conn, room_uuid, ttt.slug(), board.value());
            REQUIRE(r.has_value());

            // Retrieve game data from the database
            auto game_found = data::find_game(conn, room_uuid);
            REQUIRE(game_found);
            REQUIRE(game_found.value());

            auto game_state = game_found.value()->payload.into_proto<board_games::tic_tac_toe::Board>();
            REQUIRE(game_state.has_value());
            REQUIRE(game_state->current_turn() == 0);
            auto board_status = to_v_players(game_state->board_status());
            REQUIRE(board_status ==
                    std::vector<board_games::tic_tac_toe::Player>(9, board_games::tic_tac_toe::Player::NONE));
        });
    }
}

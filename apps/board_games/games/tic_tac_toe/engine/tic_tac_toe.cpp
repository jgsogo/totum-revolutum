#include "tic_tac_toe.h"

#include <algorithm>
#include <spdlog/spdlog.h>

#include "apps/board_games/games/tic_tac_toe/models/board.pb.h"

namespace board_games::tic_tac_toe {
    constexpr static char PLAYER1_SYMBOL = 'X';
    constexpr static char PLAYER2_SYMBOL = 'O';
    constexpr static char EMPTY_SYMBOL = ' ';

    namespace {
        static const std::vector<std::array<int, 3>> winners{
            {0, 1, 2}, {3, 4, 5}, {6, 7, 8}, // rows
            {0, 3, 6}, {1, 4, 7}, {2, 5, 8}, // cols
            {0, 4, 8}, {2, 4, 6}             // diagonals
        };

        std::optional<char> check_winner(std::string_view board_status) { return {PLAYER1_SYMBOL}; }

        bool is_draw(std::string_view board_status) {
            return std::all_of(board_status.begin(), board_status.end(), [](char c) { return c != EMPTY_SYMBOL; });
        }

    } // namespace

    tl::expected<std::string, data::Error> new_board() {
        board_game::tic_tac_toe::Board board;
        board.set_board_status({9, EMPTY_SYMBOL});
        board.set_current_turn(0);

        std::string response;
        if (!board.SerializeToString(&response)) {
            SPDLOG_ERROR("Error encoding board_game::tic_tac_toe::Board protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }
        return {response};
    }

    tl::expected<data::GameActionResponse, data::Error> run(const std::string& game_state,
                                                            const std::string& action_payload, uint8_t player_number) {
        // Decode 'game_state', check if it's possible that 'player_number' plays an action
        board_game::tic_tac_toe::Board board;
        if (!board.ParseFromString(game_state.c_str())) {
            SPDLOG_ERROR("Error decoding board_game::tic_tac_toe::Board protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }

        // Decode 'action_payload'
        board_game::tic_tac_toe::Action action;
        if (!action.ParseFromString(action_payload.c_str())) {
            SPDLOG_ERROR("Error decoding board_game::tic_tac_toe::Action protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }

        // Preconditions:
        //  - It's the players turn
        //  - Game is not finished
        //  - The cell is empty
        switch (board.turn_state_case()) {
        case board_game::tic_tac_toe::Board::TurnStateCase::kCurrentTurn:
            if (board.current_turn() != player_number) {
                SPDLOG_ERROR(
                    "Game state is expecting actions from player {}, however, game action comes from player {}",
                    board.current_turn(), player_number);
                return tl::unexpected(data::Error::GameEngineError);
            }
            break;
        case board_game::tic_tac_toe::Board::TurnStateCase::kWinner:
        case board_game::tic_tac_toe::Board::TurnStateCase::kDraw:
            SPDLOG_ERROR("Game state is finished. No action expected");
            return tl::unexpected(data::Error::GameEngineError);
        case board_game::tic_tac_toe::Board::TurnStateCase::TURN_STATE_NOT_SET:
            SPDLOG_ERROR("Error decoding board_game::tic_tac_toe::Board protobuf: 'turn_state' is not set");
            return tl::unexpected(data::Error::GameDecodeError);
        }

        if (board.board_status()[action.position()] != EMPTY_SYMBOL) {
            SPDLOG_ERROR("Cell {} is already set", action.position());
            return tl::unexpected(data::Error::GameDecodeError);
        }

        // Effects: compute new 'game_state'
        //  - Place the mark (X or O)
        //  - Check for 'win' or 'draw'
        //  - Switch current player
        std::string board_status = board.board_status();
        board_status[action.position()] = player_number == 0 ? PLAYER1_SYMBOL : PLAYER2_SYMBOL;

        data::GameActionResponse response;
        board_game::tic_tac_toe::Board new_board;
        new_board.set_board_status(board_status);
        auto winner = check_winner(board_status);
        if (winner) {
            new_board.set_winner(winner == PLAYER1_SYMBOL ? 0 : 1);
            response.new_game_state = data::GameState::FINISHED;
        } else if (is_draw(board_status)) {
            new_board.set_draw(true);
            response.new_game_state = data::GameState::FINISHED;
        } else {
            new_board.set_current_turn((player_number + 1) % 2);
        }

        // Compute return event log[s]
        board_game::tic_tac_toe::EventLog event_log;
        event_log.set_mark_placed_at_position(action.position());
        event_log.set_player(player_number);

        response.action_type = "place_mark";
        response.eventlog_type = "mark_placed";
        if (!event_log.SerializeToString(&response.eventlog_payload)) {
            SPDLOG_ERROR("Error encoding board_game::tic_tac_toe::EventLog protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }
        if (!new_board.SerializeToString(&response.new_game_state_data)) {
            SPDLOG_ERROR("Error encoding board_game::tic_tac_toe::Board protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }

        return {response};
    }
} // namespace board_games::tic_tac_toe

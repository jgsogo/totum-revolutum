#include "place_mark.h"

namespace board_games::tic_tac_toe {

    constexpr static char PLAYER_X_SYMBOL = 'X';
    constexpr static char PLAYER_O_SYMBOL = 'O';
    constexpr static char EMPTY_SYMBOL = ' ';

    Expected<std::vector<Event>> _compute_place_mark(const Board& board, const ActionPlaceMark& action,
                                                     uint8_t player_number) {
        SPDLOG_DEBUG("[tic_tac_toe] _compute_place_mark");

        // Preconditions:
        //  - It's the players turn
        //  - Game is not finished
        //  - The cell is empty
        switch (board.turn_state_case()) {
        case board_games::tic_tac_toe::Board::TurnStateCase::kCurrentTurn:
            if (board.current_turn() != player_number) {
                SPDLOG_ERROR(
                    "Game state is expecting actions from player {}, however, game action comes from player {}",
                    board.current_turn(), player_number);
                return tl::unexpected(errors::GameEngineError{std::format(
                    "Game state is expecting actions from player {}, however, game action comes from player {}",
                    board.current_turn(), player_number)});
            }
            break;
        case board_games::tic_tac_toe::Board::TurnStateCase::kWinner:
        case board_games::tic_tac_toe::Board::TurnStateCase::kDraw:
            SPDLOG_ERROR("Game state is finished. No action expected");
            return tl::unexpected(errors::GameEngineError{"Game state is finished. No action expected"});
        case board_games::tic_tac_toe::Board::TurnStateCase::TURN_STATE_NOT_SET:
            SPDLOG_ERROR("Error decoding board_games::tic_tac_toe::Board protobuf: 'turn_state' is not set");
            return tl::unexpected(errors::InvalidData{
                "Error decoding board_games::tic_tac_toe::Board protobuf: 'turn_state' is not set"});
        }

        if (board.board_status()[action.position()] != EMPTY_SYMBOL) {
            SPDLOG_ERROR("Cell {} is already set", action.position());
            return tl::unexpected(errors::InvalidAction{std::format("Cell {} is already set", action.position())});
        }

        // Events:
        //  - Place the mark (X or O)
        std::vector<Event> events;
        {
            Event event;
            EventMarkPlaced* mark_placed = event.mutable_mark_placed();
            mark_placed->set_position(action.position());
            char mark = player_number == 0 ? PLAYER_X_SYMBOL : PLAYER_O_SYMBOL;
            mark_placed->set_mark(std::string{mark});

            events.emplace_back(std::move(event));
        }
        return {std::move(events)};
    }

} // namespace board_games::tic_tac_toe

// Expected<std::pair<board_games::tic_tac_toe::Board, board_games::tic_tac_toe::Event>>
// TicTacToePlugin::_run(const board_games::tic_tac_toe::Board& game_state,
//                       const board_games::tic_tac_toe::Action& action, uint8_t player_number) {
//     SPDLOG_DEBUG("[tic_tac_toe] Play action");
//     // Preconditions:
//     //  - It's the players turn
//     //  - Game is not finished
//     //  - The cell is empty
//     switch (game_state.turn_state_case()) {
//     case board_games::tic_tac_toe::Board::TurnStateCase::kCurrentTurn:
//         if (game_state.current_turn() != player_number) {
//             SPDLOG_ERROR(
//                 "Game state is expecting actions from player {}, however, game action comes from player {}",
//                 game_state.current_turn(), player_number);
//             return tl::unexpected(errors::GameEngineError{std::format(
//                 "Game state is expecting actions from player {}, however, game action comes from player {}",
//                 game_state.current_turn(), player_number)});
//         }
//         break;
//     case board_games::tic_tac_toe::Board::TurnStateCase::kWinner:
//     case board_games::tic_tac_toe::Board::TurnStateCase::kDraw:
//         SPDLOG_ERROR("Game state is finished. No action expected");
//         return tl::unexpected(errors::GameEngineError{"Game state is finished. No action expected"});
//     case board_games::tic_tac_toe::Board::TurnStateCase::TURN_STATE_NOT_SET:
//         SPDLOG_ERROR("Error decoding board_games::tic_tac_toe::Board protobuf: 'turn_state' is not set");
//         return tl::unexpected(errors::InvalidData{
//             "Error decoding board_games::tic_tac_toe::Board protobuf: 'turn_state' is not set"});
//     }

//     if (game_state.board_status()[action.position()] != EMPTY_SYMBOL) {
//         SPDLOG_ERROR("Cell {} is already set", action.position());
//         return tl::unexpected(errors::InvalidAction{std::format("Cell {} is already set", action.position())});
//     }

//     // Effects: compute new 'game_state'
//     //  - Place the mark (X or O)
//     //  - Check for 'win' or 'draw'
//     //  - Switch current player
//     std::string board_status = game_state.board_status();
//     board_status[action.position()] = player_number == 0 ? PLAYER_X_SYMBOL : PLAYER_O_SYMBOL;

//     board_games::tic_tac_toe::Board new_board;
//     new_board.set_board_status(board_status);
//     auto winner = check_winner(board_status);
//     if (winner) {
//         board_games::tic_tac_toe::Winner* w = new_board.mutable_winner();
//         w->set_player(winner->first == PLAYER_X_SYMBOL ? 0 : 1);
//         {
//             auto* data = w->mutable_line();
//             data->Assign(winner->second.begin(), winner->second.end());
//         }
//     } else if (is_draw(board_status)) {
//         new_board.set_draw(true);
//     } else {
//         new_board.set_current_turn((player_number + 1) % 2);
//     }

//     // Compute return event log[s]
//     board_games::tic_tac_toe::Event event;
//     event.set_mark_placed_at_position(action.position());
//     event.set_player(player_number);

//     return {std::make_pair(std::move(new_board), std::move(event))};
// }

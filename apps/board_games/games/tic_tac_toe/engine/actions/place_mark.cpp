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

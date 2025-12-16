#include "mark_placed.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_mark_placed(Board&& board, const EventMarkPlaced& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_mark_placed");

        std::string board_status = board.board_status();
        board_status[event.position()] = event.mark()[0];

        board.set_board_status(board_status);
        return {std::move(board)};
    }
} // namespace board_games::tic_tac_toe

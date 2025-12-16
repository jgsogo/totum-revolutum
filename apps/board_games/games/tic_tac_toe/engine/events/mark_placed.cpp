#include "mark_placed.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_mark_placed(Board&& board, const EventMarkPlaced& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_mark_placed");

        board.mutable_board_status()->Set(event.position(), event.mark());

        return {std::move(board)};
    }
} // namespace board_games::tic_tac_toe

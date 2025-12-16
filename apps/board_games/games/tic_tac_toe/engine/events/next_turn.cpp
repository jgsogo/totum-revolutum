#include "next_turn.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_next_turn(Board&& board, const EventNextTurn& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_next_turn");
        board.set_current_turn(event.next_player());
        return {std::move(board)};
    }
} // namespace board_games::tic_tac_toe

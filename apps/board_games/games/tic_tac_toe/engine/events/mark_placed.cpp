#include "mark_placed.h"

#include "apps/board_games/games/tic_tac_toe/engine/constants.hpp"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_mark_placed(Board&& board, const EventMarkPlaced& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_mark_placed");

        std::string board_status = board.board_status();
        board_status[event.position()] = event.mark() == Player::PLAYER_X ? PLAYER_X_SYMBOL : PLAYER_O_SYMBOL;

        board.set_board_status(board_status);
        return {std::move(board)};
    }
} // namespace board_games::tic_tac_toe

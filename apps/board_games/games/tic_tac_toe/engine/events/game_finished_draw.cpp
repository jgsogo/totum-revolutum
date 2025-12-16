#include "game_finished_draw.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_game_finished_draw(Board&& board, const EventGameFinishedDraw& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_game_finished_draw");
        board.set_draw(true);
        return {std::move(board)};
    }
} // namespace board_games::tic_tac_toe

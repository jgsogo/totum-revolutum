#include "game_finished_draw.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_game_finished_draw(const Board& board, const EventGameFinishedDraw& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_game_finished_draw");
        return tl::unexpected{utils::NotImplemented{"TODO"}};
    }
} // namespace board_games::tic_tac_toe

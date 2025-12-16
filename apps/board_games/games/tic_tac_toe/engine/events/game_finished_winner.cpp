#include "game_finished_winner.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_game_finished_winner(const Board& board, const EventGameFinishedWinner& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_game_finished_winner");
        return tl::unexpected{utils::NotImplemented{"TODO"}};
    }
} // namespace board_games::tic_tac_toe

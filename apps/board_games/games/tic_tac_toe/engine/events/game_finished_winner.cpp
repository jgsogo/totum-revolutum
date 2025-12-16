#include "game_finished_winner.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_game_finished_winner(Board&& board, const EventGameFinishedWinner& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_game_finished_winner");
        Winner* winner = board.mutable_winner();
        winner->set_player(event.player());
        winner->mutable_line()->CopyFrom(event.line());
        return {std::move(board)};
    }
} // namespace board_games::tic_tac_toe

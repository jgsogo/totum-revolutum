#include "next_turn.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_new_player(Board&& board, const EventNewPlayer& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_new_player");

        (*board.mutable_players())[event.player_number()] = event.player();

        return {std::move(board)};
    }
} // namespace board_games::tic_tac_toe

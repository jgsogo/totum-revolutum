#include "next_turn.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_next_turn(const Board& board, const EventNextTurn& event) {
        SPDLOG_DEBUG("[tic_tac_toe] apply_next_turn");
        return tl::unexpected{utils::NotImplemented{"TODO"}};
    }
} // namespace board_games::tic_tac_toe

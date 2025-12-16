#pragma once

#include "apps/board_games/engine/errors/errors.hpp"

#include "apps/board_games/games/tic_tac_toe/models/board.pb.h"
#include "apps/board_games/games/tic_tac_toe/models/events.pb.h"

namespace board_games::tic_tac_toe {

    Expected<Board> apply_game_finished_draw(const Board& board, const EventGameFinishedDraw&);

}

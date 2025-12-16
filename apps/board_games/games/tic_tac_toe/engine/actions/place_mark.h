#pragma once

#include <vector>

#include "apps/board_games/engine/errors/errors.hpp"

#include "apps/board_games/games/tic_tac_toe/models/actions.pb.h"
#include "apps/board_games/games/tic_tac_toe/models/board.pb.h"
#include "apps/board_games/games/tic_tac_toe/models/events.pb.h"

namespace board_games::tic_tac_toe {

    Expected<std::vector<Event>> _compute_place_mark(const Board& board, const ActionPlaceMark& action,
                                                     uint8_t player_number);

}

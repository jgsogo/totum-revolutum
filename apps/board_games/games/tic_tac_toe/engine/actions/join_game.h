#pragma once

#include <vector>

#include "apps/board_games/engine/errors/errors.hpp"

#include "apps/board_games/games/tic_tac_toe/models/actions.pb.h"
#include "apps/board_games/games/tic_tac_toe/models/board.pb.h"
#include "apps/board_games/games/tic_tac_toe/models/events.pb.h"

namespace board_games::tic_tac_toe {

    Expected<std::pair<std::vector<Event>, uint8_t>> _compute_join_game(const Board& board,
                                                                        const ActionJoinGame& action);

}

#pragma once

#include "apps/board_games/engine/data/models/game_type.hpp"

namespace board_games::tic_tac_toe {

    static constexpr data::GameType GAME_TYPE{"tic_tac_toe"};

    constexpr static char PLAYER_X_SYMBOL = 'X';
    constexpr static char PLAYER_O_SYMBOL = 'O';
    constexpr static char EMPTY_SYMBOL = ' ';

} // namespace board_games::tic_tac_toe

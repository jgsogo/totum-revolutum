#pragma once

#include "apps/board_games/engine/data/models/game_type.hpp"

namespace board_games::tic_tac_toe {

    static constexpr data::GameType GAME_TYPE{"tic_tac_toe"};

    // tl::expected<(Status, EventLog)> run(const Status&, const GameAction&);

} // namespace board_games::tic_tac_toe

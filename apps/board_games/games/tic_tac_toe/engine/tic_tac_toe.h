#pragma once

namespace board_games::tic_tac_toe {

    tl::expected<(Status, EventLog)> run(const Status&, const GameAction&);

}

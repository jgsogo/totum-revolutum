#pragma once

#include "tl/expected.hpp"

#include "apps/board_games/engine/data/errors.h"
#include "apps/board_games/engine/data/models/game_action_response.hpp"
#include "apps/board_games/engine/data/models/game_type.hpp"

namespace board_games::tic_tac_toe {

    static constexpr data::GameType GAME_TYPE{"tic_tac_toe"};

    tl::expected<std::string, data::Error> new_board();

    tl::expected<data::GameActionResponse, data::Error> run(const std::string& game_state_payload,
                                                            const std::string& action_payload, uint8_t player_number);

} // namespace board_games::tic_tac_toe

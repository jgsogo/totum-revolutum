#pragma once

#include "tl/expected.hpp"

#include "apps/board_games/engine/data/models/game_type.hpp"

namespace board_games::tic_tac_toe {

    static constexpr data::GameType GAME_TYPE{"tic_tac_toe"};

    tl::expected<std::pair<std::string, std::string>, std::string>
    run(const std::string& game_state, const std::string& action_payload, uint8_t player_number);

} // namespace board_games::tic_tac_toe

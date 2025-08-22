#include "tic_tac_toe.h"

namespace board_games::tic_tac_toe {

    tl::expected<std::pair<std::string, std::string>, std::string>
    run(const std::string& game_state, const std::string& action_payload, uint8_t player_number) {
        return tl::unexpected("Not implemented");
    }
} // namespace board_games::tic_tac_toe

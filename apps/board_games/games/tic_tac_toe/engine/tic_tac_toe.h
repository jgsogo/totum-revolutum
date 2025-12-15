#pragma once

#include "apps/board_games/engine/data/models/game_action_response.hpp"
#include "apps/board_games/engine/data/models/game_type.hpp"

#include "apps/board_games/engine/plugin_base/game_plugin.hpp"
#include "apps/board_games/games/tic_tac_toe/models/board.pb.h"

namespace board_games::tic_tac_toe {

    class TicTacToePlugin final
        : public engine::GamePlugin<board_games::tic_tac_toe::Board, board_games::tic_tac_toe::Action,
                                    board_games::tic_tac_toe::Event> {
        using GamePlugin = engine::GamePlugin<board_games::tic_tac_toe::Board, board_games::tic_tac_toe::Action,
                                              board_games::tic_tac_toe::Event>;

      public:
        TicTacToePlugin();

      protected:
        std::string_view get_action_type(const board_games::tic_tac_toe::Action& action) const override;
        std::string_view get_event_type(const board_games::tic_tac_toe::Event& event) const override;
        data::GameState get_game_state(const board_games::tic_tac_toe::Board& game_state) const override;
        Expected<board_games::tic_tac_toe::Board> _new_board() override;
        Expected<std::pair<board_games::tic_tac_toe::Board, board_games::tic_tac_toe::Event>>
        _run(const board_games::tic_tac_toe::Board& game_state, const board_games::tic_tac_toe::Action& action,
             uint8_t player_number) override;
    };

} // namespace board_games::tic_tac_toe

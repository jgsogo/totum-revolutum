#pragma once

#include "tl/expected.hpp"

#include "apps/board_games/engine/data/errors.h"
#include "apps/board_games/engine/data/models/game_action_response.hpp"
#include "apps/board_games/engine/data/models/game_type.hpp"

#include "apps/board_games/engine/game_plugin.hpp"
#include "apps/board_games/games/tic_tac_toe/models/board.pb.h"

namespace board_games::tic_tac_toe {

    static constexpr data::GameType GAME_TYPE{"tic_tac_toe"};

    class TicTacToePlugin final
        : public engine::GamePlugin<board_game::tic_tac_toe::Board, board_game::tic_tac_toe::Action,
                                    board_game::tic_tac_toe::EventLog> {
        using GamePlugin = engine::GamePlugin<board_game::tic_tac_toe::Board, board_game::tic_tac_toe::Action,
                                              board_game::tic_tac_toe::EventLog>;

      public:
        TicTacToePlugin();

      protected:
        std::string_view get_action_type(const board_game::tic_tac_toe::Action& action) const override;
        std::string_view get_eventlog_type(const board_game::tic_tac_toe::EventLog& eventlog) const override;
        data::GameState get_game_state(const board_game::tic_tac_toe::Board& game_state) const override;
        tl::expected<board_game::tic_tac_toe::Board, data::Error> _new_board() override;
        tl::expected<std::pair<board_game::tic_tac_toe::Board, board_game::tic_tac_toe::EventLog>, data::Error>
        _run(board_game::tic_tac_toe::Board&& game_state, board_game::tic_tac_toe::Action&& action,
             uint8_t player_number) override;
    };

    tl::expected<data::GameStatePayload, data::Error> new_board();

    tl::expected<data::GameActionResponse, data::Error> run(const data::GameStatePayload& game_state_payload,
                                                            const data::GameActionPayload& action_payload,
                                                            uint8_t player_number);

} // namespace board_games::tic_tac_toe

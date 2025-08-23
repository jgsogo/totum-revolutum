#pragma once

#include "apps/board_games/engin/game_plugin.hpp"
#include "apps/board_games/games/tic_tac_toe/models/board.pb.h"

namespace board_games::tic_tac_toe {


    class TicTacToePlugin final: public engine::GamePlugin<board_game::tic_tac_toe::Board, board_game::tic_tac_toe::Action, board_game::tic_tac_toe::EventLog> {
        using GamePlugin = engine::GamePlugin<board_game::tic_tac_toe::Board, board_game::tic_tac_toe::Action, board_game::tic_tac_toe::EventLog>;
        public:
            explicit GamePlugin(std::string&& slug, std::string&& name, std::string&& description) : GamePlugin{std::move(slug), std::move(name), std::move(description)} {}

            virtual std::string_view get_action_type(const TGameActionProto& action) const = 0;
            virtual std::string_view get_eventlog_type(const TEventLogProto& eventlog) const = 0;
            virtual data::GameState get_game_state(const TGameStateProto& game_state) const = 0;
            virtual tl::expected<TGameStateProto, data::Error> _new_board() = 0;
            virtual tl::expected<std::pair<TGameStateProto, TEventLogProto>, data::Error> _run(TGameStateProto&& game_state, TGameActionProto&& action, uint8_t player_number) = 0;
    }

}
#pragma once

#include "apps/board_games/engine/data/models/game_action_response.hpp"
#include "apps/board_games/engine/data/models/game_type.hpp"
#include "apps/board_games/engine/errors/errors.hpp"

#include "apps/board_games/engine/game_plugin.hpp"
#include "apps/board_games/games/ticket_to_ride/models/board.pb.h"

namespace board_games::ticket_to_ride {

    class TicketToRidePlugin final : public engine::GamePlugin<Board, Action, EventLog> {
        using GamePlugin = engine::GamePlugin<Board, Action, EventLog>;

      public:
        TicketToRidePlugin();

      protected:
        std::string_view get_action_type(const Action& action) const override;
        std::string_view get_eventlog_type(const EventLog& eventlog) const override;
        data::GameState get_game_state(const Board& game_state) const override;
        Expected<Board> _new_board() override;
        Expected<std::pair<Board, EventLog>> _run(const Board& game_state, const Action& action,
                                                  uint8_t player_number) override;
    };

} // namespace board_games::ticket_to_ride

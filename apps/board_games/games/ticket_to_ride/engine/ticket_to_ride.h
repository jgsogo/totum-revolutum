#pragma once

#include "apps/board_games/engine/data/models/game_action_response.hpp"
#include "apps/board_games/engine/data/models/game_type.hpp"
#include "apps/board_games/engine/errors/errors.hpp"

#include "apps/board_games/engine/plugin_base/game_plugin.hpp"
#include "apps/board_games/games/ticket_to_ride/models/actions.pb.h"
#include "apps/board_games/games/ticket_to_ride/models/event_log.pb.h"
#include "apps/board_games/games/ticket_to_ride/models/game.pb.h"

namespace board_games::ticket_to_ride {

    class TicketToRidePlugin final : public engine::GamePlugin<GameState, Action, Event> {
        using GamePlugin = engine::GamePlugin<GameState, Action, Event>;

      public:
        TicketToRidePlugin();

      protected:
        std::string_view get_action_type(const Action& action) const override;
        std::string_view get_event_type(const Event& event) const override;
        data::GameState get_game_state(const GameState& game_state) const override;
        Expected<GameState> _new_board() override;

        // Returns the events that are triggered by the given action on the given board.
        Expected<std::vector<Event>> _compute_events(const GameState& board, const Action& action,
                                                     uint8_t player_number) override final;

        // Returns events generated after a turn has finished (it also checks win condition)
        Expected<std::vector<Event>> _end_turn(const GameState& board) override final;

        // Applies the given event on the given board, and return the new state for the board.
        Expected<GameState> _apply_event(GameState&& board, const Event& event) override final;
    };

} // namespace board_games::ticket_to_ride

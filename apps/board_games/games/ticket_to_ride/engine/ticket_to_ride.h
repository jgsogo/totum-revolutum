#pragma once

#include "apps/board_games/engine/data/models/game_action_response.hpp"
#include "apps/board_games/engine/data/models/game_type.hpp"
#include "apps/board_games/engine/errors/errors.hpp"

#include "apps/board_games/engine/plugin_base/game_plugin.hpp"
#include "apps/board_games/games/ticket_to_ride/models/actions.pb.h"
#include "apps/board_games/games/ticket_to_ride/models/event_log.pb.h"
#include "apps/board_games/games/ticket_to_ride/models/game.pb.h"

namespace board_games::ticket_to_ride {

    class TicketToRidePlugin final : public engine::GamePlugin<GameState, Action, event> {
        using GamePlugin = engine::GamePlugin<GameState, Action, event>;

      public:
        TicketToRidePlugin();

      protected:
        std::string_view get_action_type(const Action& action) const override;
        std::string_view get_event_type(const event& event) const override;
        data::GameState get_game_state(const GameState& game_state) const override;
        Expected<GameState> _new_board() override;
        Expected<std::pair<GameState, event>> _run(const GameState& game_state, const Action& action,
                                                   uint8_t player_number) override;
    };

} // namespace board_games::ticket_to_ride

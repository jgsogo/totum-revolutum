#include "ticket_to_ride.h"

#include <spdlog/spdlog.h>

namespace board_games::ticket_to_ride {
    static constexpr data::GameType GAME_TYPE{"ticket_to_ride"};

    static constexpr std::string_view ACTION_PLACE_MARK{"move_action"};

    static constexpr std::string_view EVENT_MARK_PLACED{"move_action"};

    TicketToRidePlugin::TicketToRidePlugin()
        : TicketToRidePlugin::GamePlugin{GAME_TYPE, std::string{"Ticket to Ride"},
                                         std::string{"Ticket to Ride board game"}} {};

    std::string_view TicketToRidePlugin::get_action_type(const Action& action) const {
        SPDLOG_ERROR("[ticket_to_ride] Not implemented");
        return ACTION_PLACE_MARK;
    }

    std::string_view TicketToRidePlugin::get_eventlog_type(const EventLog& eventlog) const {
        SPDLOG_ERROR("[ticket_to_ride] Not implemented");
        return EVENT_MARK_PLACED;
    }

    data::GameState TicketToRidePlugin::get_game_state(const Board& game_state) const {
        SPDLOG_ERROR("[ticket_to_ride] Not implemented");
        return data::GameState::WAITING;
    }

    Expected<Board> TicketToRidePlugin::_new_board() {
        SPDLOG_ERROR("[ticket_to_ride] Not implemented");
        return tl::unexpected(utils::NotImplemented{"WIP"});
    }

    Expected<std::pair<Board, EventLog>> TicketToRidePlugin::_run(const Board& game_state, const Action& action,
                                                                  uint8_t player_number) {
        SPDLOG_ERROR("[ticket_to_ride] Not implemented");
        return tl::unexpected(utils::NotImplemented{"WIP"});
    }
} // namespace board_games::ticket_to_ride

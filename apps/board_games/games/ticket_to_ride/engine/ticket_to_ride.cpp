#include "ticket_to_ride.h"

#include <spdlog/spdlog.h>

#include "apps/board_games/games/ticket_to_ride/engine/decks/destination_deck.h"
#include "apps/board_games/games/ticket_to_ride/engine/decks/train_deck.h"
#include "apps/board_games/games/ticket_to_ride/engine/maps/usa/usa.h"
#include "apps/board_games/games/ticket_to_ride/models/game.pb.h"

namespace board_games::ticket_to_ride {
    static constexpr data::GameType GAME_TYPE{"ticket_to_ride"};

    TicketToRidePlugin::TicketToRidePlugin()
        : TicketToRidePlugin::GamePlugin{GAME_TYPE, std::string{"Ticket to Ride"},
                                         std::string{"Ticket to Ride board game"}} {};

    data::GameState TicketToRidePlugin::get_game_state(const GameState& game_state) const {
        SPDLOG_ERROR("[ticket_to_ride] Not implemented");
        return data::GameState::WAITING;
    }

    Expected<GameState> TicketToRidePlugin::_new_board() {
        SPDLOG_DEBUG("[ticket_to_ride] Return new board");
        GameState game;

        MapData* map = game.mutable_map();
        populate_usa_map(*map);

        BoardState* board = game.mutable_board();
        TrainDeck* train_deck = board->mutable_train_deck();
        populate_train_deck(*train_deck, 0); // TODO: Provide a seed
        DestinationDeck* destination_deck = board->mutable_destination_deck();
        populate_destination_deck(*destination_deck, 0); // TODO: Provide a seed

        return game;
    }

    Expected<std::vector<Event>> TicketToRidePlugin::_compute_events(const GameState& board, const Action& action,
                                                                     uint8_t player_number) {
        SPDLOG_ERROR("[ticket_to_ride] _compute_events");
        return tl::unexpected(utils::NotImplemented{"WIP"});
    }

    Expected<std::vector<Event>> TicketToRidePlugin::_end_turn(const GameState& board) {
        SPDLOG_ERROR("[ticket_to_ride] _end_turn");
        return tl::unexpected(utils::NotImplemented{"WIP"});
    }

    Expected<GameState> TicketToRidePlugin::_apply_event(GameState&& board, const Event& event) {
        SPDLOG_ERROR("[ticket_to_ride] _apply_event");
        return tl::unexpected(utils::NotImplemented{"WIP"});
    }

} // namespace board_games::ticket_to_ride

#include "join_game.h"

namespace board_games::tic_tac_toe {

    Expected<std::pair<std::vector<Event>, uint8_t>> _compute_join_game(const Board& board,
                                                                        const ActionJoinGame& action) {
        SPDLOG_DEBUG("[tic_tac_toe] _compute_join_game");

        // Preconditions:
        //  - Check if we can accept more players
        if (board.players_size() >= 2) {
            return tl::unexpected(errors::InvalidAction{"There are already two players"});
        }

        //  - Check if the player is already taken
        auto found = std::find_if(board.players().begin(), board.players().end(),
                                  [&action](const auto& kv) { return kv.second == action.player(); });
        if (found != board.players().end()) {
            return tl::unexpected(errors::InvalidAction{
                std::format("Player {} is already taken", action.player() == Player::PLAYER_X ? "X" : "O")});
        }

        // Events:
        //  - Insert with the next player number
        uint8_t player_number = board.players_size();
        std::vector<Event> events;
        {
            Event event;
            EventNewPlayer* new_player = event.mutable_new_player();
            new_player->set_player(action.player());
            new_player->set_player_number(player_number);

            events.emplace_back(std::move(event));
        }
        return {std::make_pair(std::move(events), player_number)};
    }

} // namespace board_games::tic_tac_toe

#include "tic_tac_toe.h"

#include <algorithm>
#include <spdlog/spdlog.h>

#include "apps/board_games/games/tic_tac_toe/engine/actions/join_game.h"
#include "apps/board_games/games/tic_tac_toe/engine/actions/place_mark.h"

#include "apps/board_games/games/tic_tac_toe/engine/events/game_finished_draw.h"
#include "apps/board_games/games/tic_tac_toe/engine/events/game_finished_winner.h"
#include "apps/board_games/games/tic_tac_toe/engine/events/mark_placed.h"
#include "apps/board_games/games/tic_tac_toe/engine/events/new_player.h"
#include "apps/board_games/games/tic_tac_toe/engine/events/next_turn.h"

#include "constants.hpp"

namespace board_games::tic_tac_toe {

    namespace {

        std::optional<std::pair<Player, std::array<int, 3>>>
        _check_winner(const google::protobuf::RepeatedField<int>& board_status) {
            static const std::vector<std::array<int, 3>> winners{
                {0, 1, 2}, {3, 4, 5}, {6, 7, 8}, // rows
                {0, 3, 6}, {1, 4, 7}, {2, 5, 8}, // cols
                {0, 4, 8}, {2, 4, 6}             // diagonals
            };

            auto it = std::find_if(winners.begin(), winners.end(), [&board_status](const auto& winner_line) {
                auto& [a, b, c] = winner_line;
                return (board_status[a] != Player::NONE && board_status[a] == board_status[b] &&
                        board_status[a] == board_status[c]);
            });

            if (it != winners.end()) {
                const int& p = (*it)[0];
                Player player = board_status.at(p);
                return std::make_pair(player, *it);
            } else {
                return std::nullopt;
            };
        }

        bool _is_draw(const google::protobuf::RepeatedField<int>& board_status) {
            return std::all_of(board_status.begin(), board_status.end(), [](auto p) { return p != Player::NONE; });
        }

    } // namespace

    TicTacToePlugin::TicTacToePlugin()
        : TicTacToePlugin::GamePlugin{GAME_TYPE, std::string{"Tic-Tac-Toe"},
                                      std::string{"Basic Tic-Tac-Toe board game"}} {};

    data::GameState TicTacToePlugin::get_game_state(const board_games::tic_tac_toe::Board& game_state) const {
        switch (game_state.turn_state_case()) {
        case board_games::tic_tac_toe::Board::TurnStateCase::kCurrentTurn:
            return data::GameState::PLAYING;
        case board_games::tic_tac_toe::Board::TurnStateCase::kWinner:
        case board_games::tic_tac_toe::Board::TurnStateCase::kDraw:
            return data::GameState::FINISHED;
        case board_games::tic_tac_toe::Board::TurnStateCase::TURN_STATE_NOT_SET:
            return data::GameState::WAITING;
        }
    }

    Expected<board_games::tic_tac_toe::Board> TicTacToePlugin::_new_board() {
        SPDLOG_DEBUG("[tic_tac_toe] Return new board");
        board_games::tic_tac_toe::Board board;
        board.mutable_board_status()->Resize(9, Player::NONE);
        board.set_current_turn(Player::PLAYER_X);
        return {std::move(board)};
    }

    Expected<std::pair<std::vector<Event>, uint8_t>> TicTacToePlugin::_join_game(const Board& board,
                                                                                 const Action& action) {
        SPDLOG_DEBUG("[tic_tac_toe] _join_game");
        switch (action.action_case()) {
        case Action::ActionCase::kJoinGame:
            return _compute_join_game(board, action.join_game());

        // Every other action is unexpected for a _join_game
        case Action::ActionCase::kPlaceMark:
            return tl::unexpected(
                errors::LogicalError{"kPlaceMark action is not expected in the '_join_game' function"});
        case Action::ActionCase::ACTION_NOT_SET: {
            SPDLOG_ERROR("Trying to execute an action, but action is not set");
            return tl::unexpected(errors::LogicalError{"Trying to execute an action, but action is not set"});
        }
        }
    }

    Expected<std::vector<Event>> TicTacToePlugin::_compute_events(const Board& board, const Action& action,
                                                                  uint8_t player_number) {
        SPDLOG_DEBUG("[tic_tac_toe] _compute_events");
        switch (action.action_case()) {
        case Action::ActionCase::kPlaceMark:
            return _compute_place_mark(board, action.place_mark(), player_number);

        // These actions are unexpected here
        case Action::ActionCase::kJoinGame:
            return tl::unexpected(
                errors::LogicalError{"kJoinGame action is not expected in the '_compute_events' function"});
        case Action::ActionCase::ACTION_NOT_SET: {
            SPDLOG_ERROR("Trying to execute an action, but action is not set");
            return tl::unexpected(errors::LogicalError{"Trying to execute an action, but action is not set"});
        }
        }
    }

    Expected<std::vector<Event>> TicTacToePlugin::_end_turn(const Board& board) {
        SPDLOG_DEBUG("[tic_tac_toe] _end_turn");
        Event event;
        auto winner = _check_winner(board.board_status());
        if (winner) {
            EventGameFinishedWinner* winner_event = event.mutable_game_finished_winner();
            winner_event->set_player(winner->first);
            auto* data = winner_event->mutable_line();
            data->Assign(winner->second.begin(), winner->second.end());
        } else if (_is_draw(board.board_status())) {
            auto _ = event.mutable_game_finished_draw();
        } else {
            EventNextTurn* next_turn_event = event.mutable_next_turn();
            next_turn_event->set_next_player(board.current_turn() == Player::PLAYER_X ? Player::PLAYER_O
                                                                                      : Player::PLAYER_X);
        }

        return {{std::move(event)}};
    }

    Expected<Board> TicTacToePlugin::_apply_event(Board&& board, const Event& event) {
        SPDLOG_DEBUG("[tic_tac_toe] _apply_event");

        switch (event.event_case()) {
        case Event::EventCase::kMarkPlaced:
            return apply_mark_placed(std::move(board), event.mark_placed());
        case Event::EventCase::kGameFinishedWinner:
            return apply_game_finished_winner(std::move(board), event.game_finished_winner());
        case Event::EventCase::kGameFinishedDraw:
            return apply_game_finished_draw(std::move(board), event.game_finished_draw());
        case Event::EventCase::kNextTurn:
            return apply_next_turn(std::move(board), event.next_turn());
        case Event::EventCase::kNewPlayer:
            return apply_new_player(std::move(board), event.new_player());

        case Event::EventCase::EVENT_NOT_SET: {
            SPDLOG_ERROR("Trying to apply an event, but the event is not set");
            return tl::unexpected(errors::LogicalError{"Trying to apply an event, but the event is not set"});
        }
        }
    }

} // namespace board_games::tic_tac_toe

#include "tic_tac_toe.h"

#include <algorithm>
#include <spdlog/spdlog.h>

#include "apps/board_games/games/tic_tac_toe/engine/actions/place_mark.h"

#include "apps/board_games/games/tic_tac_toe/engine/events/game_finished_draw.h"
#include "apps/board_games/games/tic_tac_toe/engine/events/game_finished_winner.h"
#include "apps/board_games/games/tic_tac_toe/engine/events/mark_placed.h"
#include "apps/board_games/games/tic_tac_toe/engine/events/next_turn.h"

#include "constants.hpp"

namespace board_games::tic_tac_toe {

    namespace {

        std::optional<std::pair<char, std::array<int, 3>>> check_winner(std::string_view board_status) {
            static const std::vector<std::array<int, 3>> winners{
                {0, 1, 2}, {3, 4, 5}, {6, 7, 8}, // rows
                {0, 3, 6}, {1, 4, 7}, {2, 5, 8}, // cols
                {0, 4, 8}, {2, 4, 6}             // diagonals
            };

            auto it = std::find_if(winners.begin(), winners.end(), [&board_status](const auto& winner_line) {
                auto& [a, b, c] = winner_line;
                return (board_status[a] != EMPTY_SYMBOL && board_status[a] == board_status[b] &&
                        board_status[a] == board_status[c]);
            });

            if (it != winners.end()) {
                const int& p = (*it)[0];
                return std::make_pair(board_status.at(p), *it);
            } else {
                return std::nullopt;
            };
        }

        bool is_draw(std::string_view board_status) {
            return std::all_of(board_status.begin(), board_status.end(), [](char c) { return c != EMPTY_SYMBOL; });
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
        board.set_board_status(std::string(9, EMPTY_SYMBOL));
        board.set_current_turn(0);
        return {std::move(board)};
    }

    Expected<std::vector<Event>> TicTacToePlugin::_compute_events(const Board& board, const Action& action,
                                                                  uint8_t player_number) {
        SPDLOG_DEBUG("[tic_tac_toe] _compute_events");
        switch (action.action_case()) {
        case Action::ActionCase::kPlaceMark:
            return _compute_place_mark(board, action.place_mark(), player_number);
        case Action::ActionCase::ACTION_NOT_SET: {
            SPDLOG_ERROR("Trying to execute an action, but action is not set");
            return tl::unexpected(errors::LogicalError{"Trying to execute an action, but action is not set"});
        }
        }
    }

    Expected<std::vector<Event>> TicTacToePlugin::_end_turn(const Board& board) {
        SPDLOG_DEBUG("[tic_tac_toe] _end_turn");
        Event event;
        auto winner = check_winner(board.board_status());
        if (winner) {
            EventGameFinishedWinner* winner_event = event.mutable_game_finished_winner();
            winner_event->set_player(winner->first == PLAYER_X_SYMBOL ? 0 : 1);
            auto* data = winner_event->mutable_line();
            data->Assign(winner->second.begin(), winner->second.end());
        } else if (is_draw(board.board_status())) {
            auto _ = event.mutable_game_finished_draw();
        } else {
            EventNextTurn* next_turn_event = event.mutable_next_turn();
            next_turn_event->set_next_player((board.current_turn() + 1) % 2);
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

        case Event::EventCase::EVENT_NOT_SET: {
            SPDLOG_ERROR("Trying to apply an event, but the event is not set");
            return tl::unexpected(errors::LogicalError{"Trying to apply an event, but the event is not set"});
        }
        }
    }

} // namespace board_games::tic_tac_toe

#include "tic_tac_toe.h"

#include <algorithm>
#include <spdlog/spdlog.h>

#include "apps/board_games/games/tic_tac_toe/models/board.pb.h"

namespace board_games::tic_tac_toe {
    static constexpr data::GameType GAME_TYPE{"tic_tac_toe"};

    constexpr static char PLAYER_X_SYMBOL = 'X';
    constexpr static char PLAYER_O_SYMBOL = 'O';
    constexpr static char EMPTY_SYMBOL = ' ';

    static constexpr std::string_view ACTION_PLACE_MARK{"move_action"};

    static constexpr std::string_view EVENT_MARK_PLACED{"move_action"};

    namespace {
        static const std::vector<std::array<int, 3>> winners{
            {0, 1, 2}, {3, 4, 5}, {6, 7, 8}, // rows
            {0, 3, 6}, {1, 4, 7}, {2, 5, 8}, // cols
            {0, 4, 8}, {2, 4, 6}             // diagonals
        };

        std::optional<std::pair<char, std::array<int, 3>>> check_winner(std::string_view board_status) {
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

    std::string_view TicTacToePlugin::get_action_type(const board_game::tic_tac_toe::Action& action) const {
        return ACTION_PLACE_MARK;
    }

    std::string_view TicTacToePlugin::get_event_type(const board_game::tic_tac_toe::event& event) const {
        return EVENT_MARK_PLACED;
    }

    data::GameState TicTacToePlugin::get_game_state(const board_game::tic_tac_toe::Board& game_state) const {
        switch (game_state.turn_state_case()) {
        case board_game::tic_tac_toe::Board::TurnStateCase::kCurrentTurn:
            return data::GameState::PLAYING;
        case board_game::tic_tac_toe::Board::TurnStateCase::kWinner:
        case board_game::tic_tac_toe::Board::TurnStateCase::kDraw:
            return data::GameState::FINISHED;
        case board_game::tic_tac_toe::Board::TurnStateCase::TURN_STATE_NOT_SET:
            return data::GameState::WAITING;
        }
    }

    Expected<board_game::tic_tac_toe::Board> TicTacToePlugin::_new_board() {
        SPDLOG_DEBUG("[tic_tac_toe] Return new board");
        board_game::tic_tac_toe::Board board;
        board.set_board_status(std::string(9, EMPTY_SYMBOL));
        board.set_current_turn(0);
        return {std::move(board)};
    }

    Expected<std::pair<board_game::tic_tac_toe::Board, board_game::tic_tac_toe::event>>
    TicTacToePlugin::_run(const board_game::tic_tac_toe::Board& game_state,
                          const board_game::tic_tac_toe::Action& action, uint8_t player_number) {
        SPDLOG_DEBUG("[tic_tac_toe] Play action");
        // Preconditions:
        //  - It's the players turn
        //  - Game is not finished
        //  - The cell is empty
        switch (game_state.turn_state_case()) {
        case board_game::tic_tac_toe::Board::TurnStateCase::kCurrentTurn:
            if (game_state.current_turn() != player_number) {
                SPDLOG_ERROR(
                    "Game state is expecting actions from player {}, however, game action comes from player {}",
                    game_state.current_turn(), player_number);
                return tl::unexpected(errors::GameEngineError{std::format(
                    "Game state is expecting actions from player {}, however, game action comes from player {}",
                    game_state.current_turn(), player_number)});
            }
            break;
        case board_game::tic_tac_toe::Board::TurnStateCase::kWinner:
        case board_game::tic_tac_toe::Board::TurnStateCase::kDraw:
            SPDLOG_ERROR("Game state is finished. No action expected");
            return tl::unexpected(errors::GameEngineError{"Game state is finished. No action expected"});
        case board_game::tic_tac_toe::Board::TurnStateCase::TURN_STATE_NOT_SET:
            SPDLOG_ERROR("Error decoding board_game::tic_tac_toe::Board protobuf: 'turn_state' is not set");
            return tl::unexpected(
                errors::InvalidData{"Error decoding board_game::tic_tac_toe::Board protobuf: 'turn_state' is not set"});
        }

        if (game_state.board_status()[action.position()] != EMPTY_SYMBOL) {
            SPDLOG_ERROR("Cell {} is already set", action.position());
            return tl::unexpected(errors::InvalidAction{std::format("Cell {} is already set", action.position())});
        }

        // Effects: compute new 'game_state'
        //  - Place the mark (X or O)
        //  - Check for 'win' or 'draw'
        //  - Switch current player
        std::string board_status = game_state.board_status();
        board_status[action.position()] = player_number == 0 ? PLAYER_X_SYMBOL : PLAYER_O_SYMBOL;

        board_game::tic_tac_toe::Board new_board;
        new_board.set_board_status(board_status);
        auto winner = check_winner(board_status);
        if (winner) {
            board_game::tic_tac_toe::Winner* w = new_board.mutable_winner();
            w->set_player(winner->first == PLAYER_X_SYMBOL ? 0 : 1);
            {
                auto* data = w->mutable_line();
                data->Assign(winner->second.begin(), winner->second.end());
            }
        } else if (is_draw(board_status)) {
            new_board.set_draw(true);
        } else {
            new_board.set_current_turn((player_number + 1) % 2);
        }

        // Compute return event log[s]
        board_game::tic_tac_toe::event event_log;
        event_log.set_mark_placed_at_position(action.position());
        event_log.set_player(player_number);

        return {std::make_pair(std::move(new_board), std::move(event_log))};
    }
} // namespace board_games::tic_tac_toe

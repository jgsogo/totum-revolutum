#include "tic_tac_toe.h"

#include <algorithm>
#include <spdlog/spdlog.h>

#include "apps/board_games/games/tic_tac_toe/models/board.pb.h"

namespace board_games::tic_tac_toe {
    // static constexpr data::GameType GAME_TYPE{"tic_tac_toe"};

    constexpr static char PLAYER1_SYMBOL = 'X';
    constexpr static char PLAYER2_SYMBOL = 'O';
    constexpr static char EMPTY_SYMBOL = ' ';

    static constexpr std::string_view ACTION_PLACE_MARK{"move_action"};

    static constexpr std::string_view EVENT_MARK_PLACED{"move_action"};

    namespace {
        static const std::vector<std::array<int, 3>> winners{
            {0, 1, 2}, {3, 4, 5}, {6, 7, 8}, // rows
            {0, 3, 6}, {1, 4, 7}, {2, 5, 8}, // cols
            {0, 4, 8}, {2, 4, 6}             // diagonals
        };

        std::optional<char> check_winner(std::string_view board_status) { return {PLAYER1_SYMBOL}; }

        bool is_draw(std::string_view board_status) {
            return std::all_of(board_status.begin(), board_status.end(), [](char c) { return c != EMPTY_SYMBOL; });
        }

    } // namespace

    TicTacToePlugin::TicTacToePlugin()
        : TicTacToePlugin::GamePlugin{std::string{GAME_TYPE}, std::string{"Tic-Tac-Toe"},
                                      std::string{"Basic Tic-Tac-Toe board game"}} {};

    std::string_view TicTacToePlugin::get_action_type(const board_game::tic_tac_toe::Action& action) const {
        return ACTION_PLACE_MARK;
    }
    std::string_view TicTacToePlugin::get_eventlog_type(const board_game::tic_tac_toe::EventLog& eventlog) const {
        return EVENT_MARK_PLACED;
    }
    data::GameState TicTacToePlugin::get_game_state(const board_game::tic_tac_toe::Board& game_state) const {
        return data::GameState::PLAYING;
    }
    tl::expected<board_game::tic_tac_toe::Board, data::Error> TicTacToePlugin::_new_board() {
        return tl::unexpected(data::Error::NotImplemented);
    }
    tl::expected<std::pair<board_game::tic_tac_toe::Board, board_game::tic_tac_toe::EventLog>, data::Error>
    TicTacToePlugin::_run(board_game::tic_tac_toe::Board&& game_state, board_game::tic_tac_toe::Action&& action,
                          uint8_t player_number) {
        return tl::unexpected(data::Error::NotImplemented);
    }

    tl::expected<data::GameStatePayload, data::Error> new_board() {
        board_game::tic_tac_toe::Board board;
        board.set_board_status({9, EMPTY_SYMBOL});
        board.set_current_turn(0);

        std::string response;
        if (!board.SerializeToString(&response)) {
            SPDLOG_ERROR("Error encoding board_game::tic_tac_toe::Board protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }
        data::GameStatePayload game_state_payload{std::move(response)};
        return tl::expected<data::GameStatePayload, data::Error>(std::move(game_state_payload));
    }

    tl::expected<data::GameActionResponse, data::Error> run(const data::GameStatePayload& game_state,
                                                            const data::GameActionPayload& action_payload,
                                                            uint8_t player_number) {
        // Decode 'game_state', check if it's possible that 'player_number' plays an action
        board_game::tic_tac_toe::Board board;
        if (!board.ParseFromString(static_cast<std::string_view>(game_state))) {
            SPDLOG_ERROR("Error decoding board_game::tic_tac_toe::Board protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }

        // Decode 'action_payload'
        board_game::tic_tac_toe::Action action;
        if (!action.ParseFromString(static_cast<std::string_view>(action_payload))) {
            SPDLOG_ERROR("Error decoding board_game::tic_tac_toe::Action protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }

        // Preconditions:
        //  - It's the players turn
        //  - Game is not finished
        //  - The cell is empty
        switch (board.turn_state_case()) {
        case board_game::tic_tac_toe::Board::TurnStateCase::kCurrentTurn:
            if (board.current_turn() != player_number) {
                SPDLOG_ERROR(
                    "Game state is expecting actions from player {}, however, game action comes from player {}",
                    board.current_turn(), player_number);
                return tl::unexpected(data::Error::GameEngineError);
            }
            break;
        case board_game::tic_tac_toe::Board::TurnStateCase::kWinner:
        case board_game::tic_tac_toe::Board::TurnStateCase::kDraw:
            SPDLOG_ERROR("Game state is finished. No action expected");
            return tl::unexpected(data::Error::GameEngineError);
        case board_game::tic_tac_toe::Board::TurnStateCase::TURN_STATE_NOT_SET:
            SPDLOG_ERROR("Error decoding board_game::tic_tac_toe::Board protobuf: 'turn_state' is not set");
            return tl::unexpected(data::Error::GameDecodeError);
        }

        if (board.board_status()[action.position()] != EMPTY_SYMBOL) {
            SPDLOG_ERROR("Cell {} is already set", action.position());
            return tl::unexpected(data::Error::GameDecodeError);
        }

        // Effects: compute new 'game_state'
        //  - Place the mark (X or O)
        //  - Check for 'win' or 'draw'
        //  - Switch current player
        std::string board_status = board.board_status();
        board_status[action.position()] = player_number == 0 ? PLAYER1_SYMBOL : PLAYER2_SYMBOL;

        data::GameState new_game_state;
        board_game::tic_tac_toe::Board new_board;
        new_board.set_board_status(board_status);
        auto winner = check_winner(board_status);
        if (winner) {
            new_board.set_winner(winner == PLAYER1_SYMBOL ? 0 : 1);
            new_game_state = data::GameState::FINISHED;
        } else if (is_draw(board_status)) {
            new_board.set_draw(true);
            new_game_state = data::GameState::FINISHED;
        } else {
            new_board.set_current_turn((player_number + 1) % 2);
        }

        // Compute return event log[s]
        board_game::tic_tac_toe::EventLog event_log;
        event_log.set_mark_placed_at_position(action.position());
        event_log.set_player(player_number);

        std::string eventlog_payload, new_game_state_data;
        if (!event_log.SerializeToString(&eventlog_payload)) {
            SPDLOG_ERROR("Error encoding board_game::tic_tac_toe::EventLog protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }
        if (!new_board.SerializeToString(&new_game_state_data)) {
            SPDLOG_ERROR("Error encoding board_game::tic_tac_toe::Board protobuf");
            return tl::unexpected(data::Error::GameDecodeError);
        }
        data::GameActionResponse response{"place_mark", "mark_placed",
                                          data::EventLogPayload{std::move(eventlog_payload)},
                                          data::GameStatePayload{std::move(new_game_state_data)}, new_game_state};
        return {std::move(response)};
    }
} // namespace board_games::tic_tac_toe

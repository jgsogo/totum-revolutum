#pragma once

#include <spdlog/spdlog.h>
#include <string>

#include "libraries/utils/cpp/expected_type/expect.hpp"

#include "apps/board_games/engine/errors/errors.hpp"

#include "apps/board_games/engine/data/models/game_action_response.hpp"
#include "apps/board_games/engine/data/models/game_join_response.hpp"
#include "apps/board_games/engine/data/models/game_type.hpp"
#include "apps/board_games/engine/data/models/payload.hpp"

namespace engine {

    class GamePluginBase {
      public:
        GamePluginBase() = delete;
        explicit GamePluginBase(const data::GameType& slug, std::string&& name, std::string&& description)
            : _slug{slug}, _name{std::move(name)}, _description{std::move(description)} {};
        virtual ~GamePluginBase() {};

        data::GameType slug() const { return _slug; };
        std::string_view name() const { return _name; };
        std::string_view description() const { return _description; };

        virtual Expected<data::GamePayload> new_board() = 0;
        virtual Expected<data::GameActionResponse> run(const data::GamePayload& game_state_payload,
                                                       const data::ActionPayload& action_payload,
                                                       uint8_t player_number) = 0;

        virtual Expected<data::GameJoinResponse> join_game(const data::GamePayload& game_payload,
                                                           const data::ActionPayload& action_payload) = 0;

      protected:
        data::GameType _slug;
        std::string _name;
        std::string _description;
    };

    template <typename TBoard, typename TActionProto, typename TEventProto> class GamePlugin : public GamePluginBase {
      public:
        explicit GamePlugin(const data::GameType& slug, std::string&& name, std::string&& description)
            : GamePluginBase{slug, std::move(name), std::move(description)} {};

        Expected<data::GamePayload> new_board() override {
            return this->_new_board().and_then([](TBoard&& board) -> Expected<data::GamePayload> {
                auto game_payload = data::GamePayload::from_proto(board);
                if (!game_payload) {
                    SPDLOG_ERROR("Error encoding '{}' protobuf", board.GetTypeName());
                    return tl::unexpected(game_payload.error());
                }
                return Expected<data::GamePayload>{std::move(game_payload.value())};
            });
        }

        Expected<data::GameActionResponse> run(const data::GamePayload& game_payload,
                                               const data::ActionPayload& action_payload,
                                               uint8_t player_number) override {
            auto input_board = EXPECT(game_payload.into_proto<TBoard>());
            auto action = EXPECT(action_payload.into_proto<TActionProto>());

            auto [board, events] = EXPECT(
                this->_compute_events(input_board, action, player_number)
                    .and_then([input_board = std::move(input_board),
                               this](auto&& events) mutable -> Expected<std::pair<TBoard, std::vector<TEventProto>>> {
                        for (const auto& event : events) {
                            input_board = EXPECT(this->_apply_event(std::move(input_board), event));
                        }
                        return {std::make_pair(input_board, events)};
                    })
                    .and_then([this](auto&& board_and_events) -> Expected<std::pair<TBoard, std::vector<TEventProto>>> {
                        auto&& [board, events] = board_and_events;
                        auto win_events = EXPECT(this->_end_turn(board));
                        for (const auto& event : win_events) {
                            board = EXPECT(this->_apply_event(std::move(board), event));
                        }
                        events.insert(events.end(), win_events.begin(), win_events.end());
                        return {std::make_pair(board, events)};
                    }));

            data::GameActionResponse game_action_response{.action_type = action.GetDescriptor()->full_name(),
                                                          .new_game_payload =
                                                              EXPECT(data::GamePayload::from_proto(board)),
                                                          .new_game_state = this->get_game_state(board)};
            for (auto&& ev : events) {
                auto event_payload = EXPECT(data::EventPayload::from_proto(ev));
                game_action_response.events.emplace_back(
                    std::make_pair(ev.GetDescriptor()->full_name(), std::move(event_payload)));
            }
            return {std::move(game_action_response)};
        }

        Expected<data::GameJoinResponse> join_game(const data::GamePayload& game_payload,
                                                   const data::ActionPayload& action_payload) override {
            return tl::unexpected(utils::NotImplemented{"GamePlugin::join_game"});
        }

      protected:
        virtual data::GameState get_game_state(const TBoard& game_state) const = 0;
        virtual Expected<TBoard> _new_board() = 0;

        // Returns the events that are triggered by the given action on the given board.
        virtual Expected<std::vector<TEventProto>> _compute_events(const TBoard& board, const TActionProto& action,
                                                                   uint8_t player_number) = 0;

        // Returns events generated after a turn has finished (it also checks win condition)
        virtual Expected<std::vector<TEventProto>> _end_turn(const TBoard& board) = 0;

        // Applies the given event on the given board, and return the new state for the board.
        virtual Expected<TBoard> _apply_event(TBoard&& board, const TEventProto& event) = 0;
    };

} // namespace engine

#pragma once

#include <string>
// #include <google/protobuf/message_lite.h>
#include <spdlog/spdlog.h>

#include "apps/board_games/engine/errors/errors.hpp"

#include "apps/board_games/engine/data/models/game_action_response.hpp"
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

      protected:
        data::GameType _slug;
        std::string _name;
        std::string _description;
    };

    struct GameTypeHasher {
        std::size_t operator()(const data::GameType& k) const { return std::hash<std::string_view>()(k); }
    };

    using GamePluginsMap = std::unordered_map<data::GameType, std::unique_ptr<GamePluginBase>, GameTypeHasher>;

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
            // Decode 'board'
            auto board = game_payload.into_proto<TBoard>();
            if (!board) {
                SPDLOG_ERROR("Error decoding GamePayload: {}", board.error());
                return tl::unexpected(board.error());
            }

            // Decode 'action_payload'
            auto action = action_payload.into_proto<TActionProto>();
            if (!action) {
                SPDLOG_ERROR("Error decoding ActionPayload: {}", action.error());
                return tl::unexpected(action.error());
            }

            auto action_type = this->get_action_type(action.value());
            return this->_run(board.value(), action.value(), player_number)
                .and_then([&action_type, this](auto&& t) -> Expected<data::GameActionResponse> {
                    const auto& [new_game_payload, event] = t;
                    auto event_type = this->get_event_type(event);
                    auto new_board = this->get_game_state(new_game_payload);

                    auto event_payload = data::EventPayload::from_proto(event);
                    if (!event_payload) {
                        SPDLOG_ERROR("Error serializing {} into protobuf", event.GetTypeName());
                        return tl::unexpected(event_payload.error());
                    }

                    auto new_game_payload_expected = data::GamePayload::from_proto(new_game_payload);
                    if (!new_game_payload_expected) {
                        SPDLOG_ERROR("Error serializing {} into protobuf", new_game_payload.GetTypeName());
                        return tl::unexpected(new_game_payload_expected.error());
                    }

                    data::EventPayload event_{std::move(event_payload).value()};
                    data::GamePayload new_game_payload_{std::move(new_game_payload_expected).value()};

                    data::GameActionResponse game_action_response{std::string{action_type}, std::string{event_type},
                                                                  std::move(event_), std::move(new_game_payload_),
                                                                  new_board};
                    return {std::move(game_action_response)};
                });
        }

      protected:
        virtual std::string_view get_action_type(const TActionProto& action) const = 0;
        virtual std::string_view get_event_type(const TEventProto& event) const = 0;
        virtual data::GameState get_game_state(const TBoard& game_state) const = 0;
        virtual Expected<TBoard> _new_board() = 0;
        virtual Expected<std::pair<TBoard, TEventProto>> _run(const TBoard& game_state, const TActionProto& action,
                                                              uint8_t player_number) = 0;
    };

} // namespace engine

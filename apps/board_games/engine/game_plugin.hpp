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

    template <typename TGameStateProto, typename TGameActionProto, typename TEventLogProto>
    class GamePlugin : public GamePluginBase {
      public:
        explicit GamePlugin(const data::GameType& slug, std::string&& name, std::string&& description)
            : GamePluginBase{slug, std::move(name), std::move(description)} {};

        Expected<data::GamePayload> new_board() override {
            return this->_new_board().and_then([](auto&& game_state) -> Expected<data::GamePayload> {
                auto game_state_payload = data::GamePayload::from_proto(game_state);
                if (!game_state_payload) {
                    SPDLOG_ERROR("Error encoding '{}' protobuf", game_state.GetTypeName());
                    return tl::unexpected(game_state_payload.error());
                }
                return Expected<data::GamePayload>{std::move(game_state_payload.value())};
            });
        }

        Expected<data::GameActionResponse> run(const data::GamePayload& game_state_payload,
                                               const data::ActionPayload& action_payload,
                                               uint8_t player_number) override {
            // Decode 'game_state'
            auto game_state = game_state_payload.into_proto<TGameStateProto>();
            if (!game_state) {
                SPDLOG_ERROR("Error decoding GamePayload: {}", game_state.error());
                return tl::unexpected(game_state.error());
            }

            // Decode 'action_payload'
            auto action = action_payload.into_proto<TGameActionProto>();
            if (!action) {
                SPDLOG_ERROR("Error decoding ActionPayload: {}", action.error());
                return tl::unexpected(action.error());
            }

            auto action_type = this->get_action_type(action.value());
            return this->_run(game_state.value(), action.value(), player_number)
                .and_then([&action_type, this](auto&& t) -> Expected<data::GameActionResponse> {
                    const auto& [new_game_state_data, eventlog] = t;
                    auto eventlog_type = this->get_eventlog_type(eventlog);
                    auto new_game_state = this->get_game_state(new_game_state_data);

                    auto eventlog_payload = data::EventPayload::from_proto(eventlog);
                    if (!eventlog_payload) {
                        SPDLOG_ERROR("Error serializing {} into protobuf", eventlog.GetTypeName());
                        return tl::unexpected(eventlog_payload.error());
                    }

                    auto new_game_state_data_payload = data::GamePayload::from_proto(new_game_state_data);
                    if (!new_game_state_data_payload) {
                        SPDLOG_ERROR("Error serializing {} into protobuf", new_game_state_data.GetTypeName());
                        return tl::unexpected(new_game_state_data_payload.error());
                    }

                    data::EventPayload eventlog_{std::move(eventlog_payload).value()};
                    data::GamePayload new_game_state_data_{std::move(new_game_state_data_payload).value()};

                    data::GameActionResponse game_action_response{std::string{action_type}, std::string{eventlog_type},
                                                                  std::move(eventlog_), std::move(new_game_state_data_),
                                                                  new_game_state};
                    return {std::move(game_action_response)};
                });
        }

      protected:
        virtual std::string_view get_action_type(const TGameActionProto& action) const = 0;
        virtual std::string_view get_eventlog_type(const TEventLogProto& eventlog) const = 0;
        virtual data::GameState get_game_state(const TGameStateProto& game_state) const = 0;
        virtual Expected<TGameStateProto> _new_board() = 0;
        virtual Expected<std::pair<TGameStateProto, TEventLogProto>>
        _run(const TGameStateProto& game_state, const TGameActionProto& action, uint8_t player_number) = 0;
    };

} // namespace engine

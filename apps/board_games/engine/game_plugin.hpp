#pragma once

#include <string>
// #include <google/protobuf/message_lite.h>
#include <spdlog/spdlog.h>
#include <tl/expected.hpp>

#include "apps/board_games/engine/data/errors.h"
#include "apps/board_games/engine/data/models/game_action_response.hpp"
#include "apps/board_games/engine/data/models/game_type.hpp"
#include "apps/board_games/engine/data/models/payload.hpp"

namespace engine {

    // template <typename T> requires std::is_base_of_v<google::protobuf::MessageLite, T>
    // class ProtobufPayload final: public Payload {
    //     public:
    //         explicit ProtobufPayload(std::string&& payload) : Payload{std::move(payload)} {}

    //         static tl::expected<ProtobufPayload, T> from(T&& payload) {
    //             std::string serialized_payload;
    //             if (!payload.SerializeToString(&serialized_payload)) {
    //                 SPDLOG_ERROR("Error encoding '{}' protobuf", payload.GetTypeName());
    //                 return tl::unexpected(payload);
    //             }
    //             return {ProtobufPayload{serialized_payload}};
    //         }
    // };

    class GamePluginBase {
      public:
        GamePluginBase() = delete;
        explicit GamePluginBase(const data::GameType& slug, std::string&& name, std::string&& description)
            : _slug{slug}, _name{std::move(name)}, _description{std::move(description)} {};
        virtual ~GamePluginBase() {};

        data::GameType slug() const { return _slug; };
        std::string_view name() const { return _name; };
        std::string_view description() const { return _description; };

        virtual tl::expected<data::GameStatePayload, data::Error> new_board() = 0;
        virtual tl::expected<data::GameActionResponse, data::Error>
        run(const data::GameStatePayload& game_state_payload, const data::GameActionPayload& action_payload,
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

        tl::expected<data::GameStatePayload, data::Error> new_board() override {
            return this->_new_board().and_then(
                [](auto&& game_state) -> tl::expected<data::GameStatePayload, data::Error> {
                    std::vector<std::byte> serialized(game_state.ByteSizeLong());
                    if (!game_state.SerializeToArray(serialized.data(), serialized.size())) {
                        SPDLOG_ERROR("Error encoding '{}' protobuf", game_state.GetTypeName());
                        return tl::unexpected(data::Error::GameEncodeError);
                    }
                    data::GameStatePayload game_state_payload{std::move(serialized)};
                    return tl::expected<data::GameStatePayload, data::Error>{std::move(game_state_payload)};
                });
        }

        tl::expected<data::GameActionResponse, data::Error> run(const data::GameStatePayload& game_state_payload,
                                                                const data::GameActionPayload& action_payload,
                                                                uint8_t player_number) override {
            // Decode 'game_state'
            auto game_state = game_state_payload.into_proto<TGameStateProto>();
            if (!game_state.has_value()) {
                SPDLOG_ERROR("Error decoding GameStatePayload: {}", game_state.error());
                return tl::unexpected(data::Error::GameDecodeError);
            }

            // Decode 'action_payload'
            auto action = action_payload.into_proto<TGameActionProto>();
            if (!action.has_value()) {
                SPDLOG_ERROR("Error decoding GameActionPayload: {}", action.error());
                return tl::unexpected(data::Error::GameDecodeError);
            }

            auto action_type = this->get_action_type(action.value());
            return this->_run(game_state.value(), action.value(), player_number)
                .and_then([&action_type, this](auto&& t) -> tl::expected<data::GameActionResponse, data::Error> {
                    const auto& [new_game_state_data, eventlog] = t;
                    auto eventlog_type = this->get_eventlog_type(eventlog);
                    auto new_game_state = this->get_game_state(new_game_state_data);

                    std::vector<std::byte> eventlog_payload(eventlog.ByteSizeLong());
                    std::vector<std::byte> new_game_state_data_payload(new_game_state_data.ByteSizeLong());
                    if (!eventlog.SerializeToArray(eventlog_payload.data(), eventlog_payload.size())) {
                        SPDLOG_ERROR("Error serializing {} into protobuf", eventlog.GetTypeName());
                        return tl::unexpected(data::Error::GameDecodeError);
                    }
                    if (!new_game_state_data.SerializeToArray(new_game_state_data_payload.data(),
                                                              new_game_state_data_payload.size())) {
                        SPDLOG_ERROR("Error serializing {} into protobuf", new_game_state_data.GetTypeName());
                        return tl::unexpected(data::Error::GameDecodeError);
                    }

                    data::GameActionResponse game_action_response{
                        std::string{action_type}, std::string{eventlog_type},
                        data::EventLogPayload{std::move(eventlog_payload)},
                        data::GameStatePayload{std::move(new_game_state_data_payload)}, new_game_state};
                    return tl::expected<data::GameActionResponse, data::Error>{std::move(game_action_response)};
                });
        }

      protected:
        virtual std::string_view get_action_type(const TGameActionProto& action) const = 0;
        virtual std::string_view get_eventlog_type(const TEventLogProto& eventlog) const = 0;
        virtual data::GameState get_game_state(const TGameStateProto& game_state) const = 0;
        virtual tl::expected<TGameStateProto, data::Error> _new_board() = 0;
        virtual tl::expected<std::pair<TGameStateProto, TEventLogProto>, data::Error>
        _run(const TGameStateProto& game_state, const TGameActionProto& action, uint8_t player_number) = 0;
    };

} // namespace engine

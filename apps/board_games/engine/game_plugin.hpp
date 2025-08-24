#pragma once

#include <string>
// #include <google/protobuf/message_lite.h>
#include <spdlog/spdlog.h>

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
        explicit GamePluginBase(std::string&& slug, std::string&& name, std::string&& description)
            : _slug{std::move(slug)}, _name{std::move(name)}, _description{std::move(description)} {};

        std::string_view slug() const { return _slug; };
        std::string_view name() const { return _name; };
        std::string_view description() const { return _description; };

        virtual tl::expected<data::GameStatePayload, data::Error> new_board() = 0;
        virtual tl::expected<data::GameActionResponse, data::Error>
        run(const data::GameStatePayload& game_state_payload, const data::GameActionPayload& action_payload,
            uint8_t player_number) = 0;

      protected:
        std::string _slug;
        std::string _name;
        std::string _description;
    };

    template <typename TGameStateProto, typename TGameActionProto, typename TEventLogProto>
    class GamePlugin : public GamePluginBase {
      public:
        explicit GamePlugin(std::string&& slug, std::string&& name, std::string&& description)
            : GamePluginBase{std::move(slug), std::move(name), std::move(description)} {};

        tl::expected<data::GameStatePayload, data::Error> new_board() override {
            return this->_new_board().and_then(
                [](auto&& game_state) -> tl::expected<data::GameStatePayload, data::Error> {
                    std::string serialized;
                    if (!game_state.SerializeToString(&serialized)) {
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
            TGameStateProto game_state;
            if (!game_state.ParseFromString(static_cast<std::string_view>(game_state_payload))) {
                SPDLOG_ERROR("Error decoding GameStatePayload into {}", game_state.GetTypeName());
                return tl::unexpected(data::Error::GameDecodeError);
            }

            // Decode 'action_payload'
            TGameActionProto action;
            if (!action.ParseFromString(static_cast<std::string_view>(action_payload))) {
                SPDLOG_ERROR("Error decoding GameActionPayload into {}", action.GetTypeName());
                return tl::unexpected(data::Error::GameDecodeError);
            }

            auto action_type = this->get_action_type(action);
            return this->_run(std::move(game_state), std::move(action), player_number)
                .and_then([&action_type, this](auto&& t) -> tl::expected<data::GameActionResponse, data::Error> {
                    const auto& [new_game_state_data, eventlog] = t;
                    auto eventlog_type = this->get_eventlog_type(eventlog);
                    auto new_game_state = this->get_game_state(new_game_state_data);

                    std::string eventlog_payload, new_game_state_data_payload;
                    if (!eventlog.SerializeToString(&eventlog_payload)) {
                        SPDLOG_ERROR("Error serializing {} into protobuf", eventlog.GetTypeName());
                        return tl::unexpected(data::Error::GameDecodeError);
                    }
                    if (!new_game_state_data.SerializeToString(&new_game_state_data_payload)) {
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
        _run(TGameStateProto&& game_state, TGameActionProto&& action, uint8_t player_number) = 0;
    };

} // namespace engine

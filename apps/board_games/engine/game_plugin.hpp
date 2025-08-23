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

    template <typename TGameStateProto, typename TGameActionProto, typename TEventLogProto>
    class GamePlugin {
        public:
            explicit GamePlugin(std::string&& slug, std::string&& name, std::string&& description);

            std::string_view slug() const { return slug; };
            std::string_view name() const { return name; };
            std::string_view description() const { return description; };

            tl::expected<GameStatePayload, data::Error> new_board() {
                return GamePlugin::_new_board().and_then([](auto&& game_state){
                    std::string serialized;
                    if (!game_state.SerializeToString(&serialized)) {
                        SPDLOG_ERROR("Error encoding '{}' protobuf", game_state.GetTypeName());
                        return tl::unexpected(data::Error::GameEncodeError);
                    }
                    return {GameStatePayload{serialized}};
                })
            }

            tl::expected<data::GameActionResponse, data::Error> run(const GameStatePayload& game_state_payload,
                                                            const GameActionPayload& action_payload, uint8_t player_number) {
                // Decode 'game_state'
                TGameStateProto game_state;
                if (!game_state.ParseFromString(game_state_payload.c_str())) {
                    SPDLOG_ERROR("Error decoding GameStatePayload into {}", game_state.GetTypeName());
                    return tl::unexpected(data::Error::GameDecodeError);
                }

                // Decode 'action_payload'
                TGameActionProto action;
                if (!action.ParseFromString(action_payload.c_str())) {
                    SPDLOG_ERROR("Error decoding GameActionPayload into {}", action.GetTypeName());
                    return tl::unexpected(data::Error::GameDecodeError);
                }

                auto action_type = GamePlugin::get_action_type(action);
                return GamePlugin::_run(std::move(game_state), std::move(action), player_number)
                    .and_then([&action_type](auto&& t){
                        const auto& [new_game_state_data, eventlog] = t;
                        auto eventlog_type = GamePlugin::get_eventlog_type(eventlog);
                        auto new_game_state = GamePlugin::get_game_state(new_game_state_data);
                        return tl::expected{data::GameActionResponse{action_type, eventlog_type, eventlog, new_game_state_data, new_game_state}};
                    })
            }
          
        protected:
            virtual std::string_view get_action_type(const TGameActionProto& action) const = 0;
            virtual std::string_view get_eventlog_type(const TEventLogProto& eventlog) const = 0;
            virtual data::GameState get_game_state(const TGameStateProto& game_state) const = 0;
            virtual tl::expected<TGameStateProto, data::Error> _new_board() = 0;
            virtual tl::expected<std::pair<TGameStateProto, TEventLogProto>, data::Error> _run(TGameStateProto&& game_state, TGameActionProto&& action, uint8_t player_number) = 0;


        protected:
            std::string slug;
            std::string name;
            std::string description;
    }

}
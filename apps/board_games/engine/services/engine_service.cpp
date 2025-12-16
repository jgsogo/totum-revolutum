#include "engine_service.h"

#include <spdlog/spdlog.h>

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/data/room.h"

namespace services {

    EngineServiceImpl::EngineServiceImpl(utils::libpqxx::ConnectionPool& pool, const engine::GamePluginsMap& games)
        : pool{pool}, _games(games) {}

    grpc::Status EngineServiceImpl::CreateNewRoom(grpc::ServerContext* context,
                                                  const board_games::NewRoomRequest* request,
                                                  google::protobuf::Empty* response) {
        SPDLOG_DEBUG("CreateNewRoom");
        return pool.with_conn<grpc::Status>([request](pqxx::connection& conn) -> grpc::Status {
            data::RoomUUID room{std::string{request->uuid()}};
            auto r = data::insert_new_room(conn, room, request->name())
                         .and_then([&conn, &room]() { return data::notify_room_update(conn, room); })

                         .and_then([]() { return Expected<grpc::Status>{grpc::Status::OK}; })
                         .or_else([](const auto& e) {
                             auto status = grpc::Status{grpc::StatusCode::INTERNAL,
                                                        std::format("Failed to insert new room: {}", e)};
                             return Expected<grpc::Status>{status};
                         });
            return r.value();
        });
    }

    grpc::Status EngineServiceImpl::StartGame(grpc::ServerContext* context,
                                              const board_games::StartGameRequest* request,
                                              google::protobuf::Empty* response) {
        SPDLOG_DEBUG("StartGame");
        return pool.with_conn<grpc::Status>([request, this](pqxx::connection& conn) {
            data::RoomUUID room{std::string{request->room_uuid()}};
            data::GameType game_type{std::string{request->game_type()}};

            // FIXME: remove_game and start_game should go inside the same transaction
            auto r = data::remove_game(conn, room)
                         .and_then([&conn, &room, &game_type, this]() -> Expected<void> {
                             auto it = this->_games.find(game_type);
                             if (it == this->_games.end()) {
                                 SPDLOG_ERROR("Game type {} not known", game_type);
                                 return tl::unexpected{
                                     errors::InvalidData{std::format("Game type {} not known", game_type)}};
                             }
                             const auto& game = it->second;
                             return game->new_board().and_then(
                                 [&conn, &room, &game_type](data::GamePayload&& initial_board_status) {
                                     return data::start_game(conn, room, game_type, initial_board_status);
                                 });
                         })
                         .and_then([&conn, &room]() { return data::notify_room_update(conn, room); })

                         .and_then([]() { return Expected<grpc::Status>{grpc::Status::OK}; })
                         .or_else([](const auto& e) {
                             auto status =
                                 grpc::Status{grpc::StatusCode::INTERNAL, std::format("Failed to start game: {}", e)};
                             return Expected<grpc::Status>{status};
                         });
            return r.value();
        });
    }

    grpc::Status EngineServiceImpl::GetOrCreateParticipant(grpc::ServerContext* context,
                                                           const board_games::GetOrCreateParticipantRequest* request,
                                                           board_games::Participant* response) {
        SPDLOG_DEBUG("GetOrCreateParticipant");
        return pool.with_conn<grpc::Status>([request, response](pqxx::connection& conn) {
            data::RoomUUID room{std::string{request->room_uuid()}};
            data::ParticipantUUID participant{std::string{request->participant_uuid()}};

            {
                // Try to get already existing participant
                auto r = data::find_participant(conn, room, participant);
                if (r.has_value() && r.value().has_value()) {
                    data::Participant participant = r.value().value();
                    response->set_room_uuid(participant.room);
                    response->set_uuid(participant.uuid);
                    response->set_role(data::participant_role_to_string(participant.role));
                    response->set_player_number(participant.player_number);
                    return grpc::Status::OK;
                }
            }

            {
                // Add and return
                auto role_expected = data::participant_role_from_string(request->participant_role());
                if (!role_expected.has_value()) {
                    auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Failed to parse participant role"};
                    return status;
                }
                data::ParticipantRole role = role_expected.value();

                auto r = data::add_participant(conn, room, participant, role, request->player_number())
                             .and_then([response](data::Participant p) {
                                 response->set_room_uuid(p.room);
                                 response->set_uuid(p.uuid);
                                 response->set_role(data::participant_role_to_string(p.role));
                                 response->set_player_number(p.player_number);
                                 return Expected<void>{};
                             })
                             .and_then([&conn, &room]() { return data::notify_room_update(conn, room); })

                             .and_then([]() { return Expected<grpc::Status>{grpc::Status::OK}; })
                             .or_else([](const auto& e) {
                                 auto status = grpc::Status{grpc::StatusCode::INTERNAL,
                                                            std::format("Failed to add participant to room: {}", e)};
                                 return Expected<grpc::Status>{status};
                             });
                return r.value();
            }
        });
    }

    grpc::Status EngineServiceImpl::SendAction(grpc::ServerContext* context,
                                               const board_games::SendActionRequest* request,
                                               google::protobuf::Empty* response) {
        SPDLOG_DEBUG("SendAction(request.room_uuid={}, request.participant_uuid={})", request->room_uuid(),
                     request->participant_uuid());
        return pool.with_conn<grpc::Status>([request, this](pqxx::connection& conn) {
            data::RoomUUID room{std::string{request->room_uuid()}};
            data::ParticipantUUID participant{std::string{request->participant_uuid()}};

            // Get the game and the participant from 'room_uuid'
            auto res =
                data::find_game(conn, room)
                    .and_then([&conn, &room, &participant](std::optional<data::Game> game)
                                  -> Expected<std::pair<data::Game, data::Participant>> {
                        if (game) {
                            // TODO: We can check game.state. If it is FINISHED or WAITING, no actions are expected
                            return data::find_participant(conn, room, participant)
                                .and_then([&game, &room, &participant](std::optional<data::Participant> participant_opt)
                                              -> Expected<std::pair<data::Game, data::Participant>> {
                                    if (participant_opt) {
                                        std::pair<data::Game, data::Participant> data{std::move(*game),
                                                                                      participant_opt.value()};
                                        Expected<std::pair<data::Game, data::Participant>> ret{std::move(data)};
                                        return ret;
                                    } else {
                                        SPDLOG_ERROR("No participant found in room {} with uuid {}", room, participant);
                                        return tl::unexpected{errors::InvalidData{std::format(
                                            "No participant found in room {} with uuid {}", room, participant)}};
                                    }
                                });
                        } else {
                            SPDLOG_ERROR("No game found in room {}", room);
                            return tl::unexpected{errors::InvalidData{std::format("No game found in room {}", room)}};
                        }
                    })
                    // Switch based on game.game_type and execute the run function
                    .and_then(
                        [&conn, request, this](
                            const std::pair<data::Game, data::Participant>&& game_and_participant) -> Expected<void> {
                            const auto&& [game, participant] = std::move(game_and_participant);

                            // We don't care if the game is enabled or not. Maybe it's an ongoing game
                            auto it = this->_games.find(game.type);
                            if (it == this->_games.end()) {
                                SPDLOG_ERROR("Game type {} not known", game.type);
                                return tl::unexpected{
                                    errors::InvalidData{std::format("Game type {} not known", game.type)}};
                            }
                            const auto& game_plugin = it->second;
                            std::vector<std::byte> payload(request->payload().size());
                            std::memcpy(payload.data(), request->payload().data(), request->payload().size());
                            const auto& action_payload = data::ActionPayload{std::move(payload)};
                            Expected<void> r =
                                game_plugin
                                    ->run(game.payload, action_payload, participant.player_number)
                                    // Store to the database the action + new status + events
                                    // FIME: We might want to do all of this in a single transaction
                                    .and_then(
                                        [&conn, &game, &participant, &action_payload](data::GameActionResponse&& res) {
                                            return data::store_action(conn, game.id, participant.uuid, res.action_type,
                                                                      action_payload, true)
                                                .and_then([&conn, &game,
                                                           &res](const std::int64_t& action_id) -> Expected<void> {
                                                    for (auto&& ev : res.events) {
                                                        auto inserted = data::store_event(conn, game.id, ev.first,
                                                                                          ev.second, action_id);
                                                        if (!inserted) {
                                                            return inserted;
                                                        }
                                                    }
                                                    return {};
                                                })
                                                .and_then([&conn, &game, &res]() {
                                                    return data::update_game_state(conn, game.id, res.new_game_state,
                                                                                   res.new_game_payload);
                                                });
                                        })
                                    // On failure: RETURN to the user that the action could not be understood.
                                    .or_else(
                                        [&conn, &game, &participant, &action_payload](const auto& e) -> Expected<void> {
                                            SPDLOG_ERROR("Failed to apply action to the game: {}", e);
                                            std::ignore = data::store_action(conn, game.id, participant.uuid, "unknown",
                                                                             action_payload, false);
                                            return tl::unexpected{errors::InvalidAction{
                                                std::format("Failed to apply action to the game: {}", e)}};
                                        });
                            return r;
                        })
                    // Send notification to the room
                    .and_then([&conn, &room]() { return data::notify_room_update(conn, room); });

            if (res.has_value()) {
                return grpc::Status::OK;
            } else {
                auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Failed to apply action to game"};
                return status;
            }
        });
    }
} // namespace services

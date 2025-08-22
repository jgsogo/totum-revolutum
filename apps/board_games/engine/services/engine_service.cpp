#include "engine_service.h"

#include <spdlog/spdlog.h>

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/data/room.h"

#include "apps/board_games/games/tic_tac_toe/engine/tic_tac_toe.h"

namespace services {

    EngineServiceImpl::EngineServiceImpl(db::ConnectionPool& pool) : pool{pool} {}

    grpc::Status EngineServiceImpl::SubmitCommand(grpc::ServerContext* context,
                                                  const board_game::CommandRequest* request,
                                                  board_game::CommandResponse* response) {
        SPDLOG_DEBUG("SubmitCommand");
        pool.with_conn<void>([response](pqxx::connection& conn) { response->set_success(true); });
        return grpc::Status::OK;
    }

    grpc::Status EngineServiceImpl::CreateNewRoom(grpc::ServerContext* context,
                                                  const board_game::NewRoomRequest* request,
                                                  google::protobuf::Empty* response) {
        SPDLOG_DEBUG("CreateNewRoom");
        return pool.with_conn<grpc::Status>([request](pqxx::connection& conn) -> grpc::Status {
            data::RoomUUID room{std::string{request->uuid()}};
            auto r = data::insert_new_room(conn, room, request->name())
                         .and_then([&conn, &room]() { return data::notify_room_update(conn, room); })

                         .and_then([]() { return tl::expected<grpc::Status, data::Error>{grpc::Status::OK}; })
                         .or_else([](const data::Error& e) {
                             auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Failed to insert new room"};
                             return tl::expected<grpc::Status, data::Error>{status};
                         });
            return r.value();
        });
    }

    grpc::Status EngineServiceImpl::StartGame(grpc::ServerContext* context, const board_game::StartGameRequest* request,
                                              google::protobuf::Empty* response) {
        SPDLOG_DEBUG("StartGame");
        return pool.with_conn<grpc::Status>([request](pqxx::connection& conn) {
            data::RoomUUID room{std::string{request->room_uuid()}};
            data::GameType game_type{std::string{request->game_type()}};

            // FIXME: remove_game and start_game should go inside the same transaction
            auto r = data::remove_game(conn, room)
                         .and_then([&conn, &room, &game_type]() -> tl::expected<void, data::Error> {
                             if (game_type == board_games::tic_tac_toe::GAME_TYPE) {
                                 return board_games::tic_tac_toe::new_board().and_then(
                                     [&conn, &room, &game_type](const std::string& initial_board_status) {
                                         return data::start_game(conn, room, game_type, initial_board_status);
                                     });
                             } else {
                                 SPDLOG_ERROR("Game type {} not known", game_type);
                                 return tl::unexpected{data::Error::NotFound};
                             }
                         })
                         .and_then([&conn, &room]() { return data::notify_room_update(conn, room); })

                         .and_then([]() { return tl::expected<grpc::Status, data::Error>{grpc::Status::OK}; })
                         .or_else([](const data::Error& e) {
                             auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Failed to start game"};
                             return tl::expected<grpc::Status, data::Error>{status};
                         });
            return r.value();
        });
    }

    grpc::Status EngineServiceImpl::GetOrCreateParticipant(grpc::ServerContext* context,
                                                           const board_game::GetOrCreateParticipantRequest* request,
                                                           board_game::Participant* response) {
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

                auto r = data::add_participant(conn, room, participant, role)
                             .and_then([response](data::Participant p) {
                                 response->set_room_uuid(p.room);
                                 response->set_uuid(p.uuid);
                                 response->set_role(data::participant_role_to_string(p.role));
                                 response->set_player_number(p.player_number);
                                 return tl::expected<void, data::Error>{};
                             })
                             .and_then([&conn, &room]() { return data::notify_room_update(conn, room); })

                             .and_then([]() { return tl::expected<grpc::Status, data::Error>{grpc::Status::OK}; })
                             .or_else([](const data::Error& e) {
                                 auto status =
                                     grpc::Status{grpc::StatusCode::INTERNAL, "Failed to add participant to room"};
                                 return tl::expected<grpc::Status, data::Error>{status};
                             });
                return r.value();
            }
        });
    }

    grpc::Status EngineServiceImpl::SendGameAction(grpc::ServerContext* context,
                                                   const board_game::SendGameActionRequest* request,
                                                   google::protobuf::Empty* response) {
        SPDLOG_DEBUG("GetOrCreateParticipant");
        return pool.with_conn<grpc::Status>([request](pqxx::connection& conn) {
            data::RoomUUID room{std::string{request->room_uuid()}};
            data::ParticipantUUID participant{std::string{request->participant_uuid()}};

            // Get the game and the participant from 'room_uuid'
            auto res =
                data::find_game(conn, room)
                    .and_then([&conn, &room, &participant](std::optional<data::Game> game)
                                  -> tl::expected<std::pair<data::Game, data::Participant>, data::Error> {
                        if (game) {
                            // TODO: We can check game.state. If it is FINISHED or WAITING, no actions are expected
                            return data::find_participant(conn, room, participant)
                                .and_then([&game, &room, &participant](std::optional<data::Participant> participant_opt)
                                              -> tl::expected<std::pair<data::Game, data::Participant>, data::Error> {
                                    if (participant_opt) {
                                        std::pair<data::Game, data::Participant> data{game.value(),
                                                                                      participant_opt.value()};
                                        return {data};
                                    } else {
                                        SPDLOG_ERROR("No participant found in room {} with uuid {}", room, participant);
                                        return tl::unexpected{data::Error::NotFound};
                                    }
                                });
                        } else {
                            SPDLOG_ERROR("No game found in room {}", room);
                            return tl::unexpected{data::Error::NotFound};
                        }
                    })
                    // Switch based on game.game_type and execute the run function
                    .and_then([&conn, &request](std::pair<data::Game, data::Participant> game_and_participant)
                                  -> tl::expected<void, data::Error> {
                        auto [game, participant] = game_and_participant;

                        // TODO: Factory + register different games
                        if (game.type == board_games::tic_tac_toe::GAME_TYPE) {
                            return board_games::tic_tac_toe::run(game.state_data, request->payload(),
                                                                 participant.player_number)
                                // Store to the database the action + new status + event_log
                                .and_then([&conn, &game, &participant, &request](const data::GameActionResponse& res) {
                                    return data::store_action(conn, game.id, participant.uuid, res.action_type,
                                                              request->payload(), true)
                                        .and_then([&conn, &game, &res](const std::int64_t& action_id) {
                                            return data::store_eventlog(conn, game.id, res.eventlog_type,
                                                                        res.eventlog_payload, action_id);
                                        })
                                        .and_then([&conn, &game, &res]() {
                                            return data::update_game_state(conn, game.id, res.new_game_state,
                                                                           res.new_game_state_data);
                                        });
                                })
                                // On failure: RETURN to the user that the action could not be understood.
                                .or_else([&conn, &game, &participant,
                                          &request](const data::Error& e) -> tl::expected<void, data::Error> {
                                    SPDLOG_ERROR("Failed to apply action to the game");
                                    std::ignore = data::store_action(conn, game.id, participant.uuid, "unknown",
                                                                     request->payload(), false);
                                    return tl::unexpected{data::Error::GameActionFailed};
                                });
                        } else {
                            SPDLOG_ERROR("Game type {} not known", game.type);
                            return tl::unexpected{data::Error::NotFound};
                        }

                        return tl::unexpected{data::Error::NotFound};
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

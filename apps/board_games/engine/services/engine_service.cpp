#include "engine_service.h"

#include <spdlog/spdlog.h>

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/data/room.h"

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
                         .and_then([&conn, &room, &game_type]() { return data::start_game(conn, room, game_type); })
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
                auto r = data::get_participant(conn, room, participant);
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

            // Get the game from 'room_uuid'

            // Check the participant data (mostly interested in player-number)

            // Switch based on game.game_type and execute the run function

            // On failure: RETURN to the user that the action could not be understood.

            // Store to the database the action + new status + event_log
            //  - store if the 'action' was successfully applied or not
            //  - associate the 'event_log' to the action that generated it (also failures).

            // Send notification to the room

            auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Not implemented"};
            return status;
        });
    }
} // namespace services

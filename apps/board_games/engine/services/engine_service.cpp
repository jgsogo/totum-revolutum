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
            auto r = data::insert_new_room(conn, data::RoomUUID{std::string{request->uuid()}}, request->name())
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

            auto r = data::remove_game(conn, room)
                         .and_then([&conn, &room, &game_type]() { return data::start_game(conn, room, game_type); })
                         .and_then([]() { return tl::expected<grpc::Status, data::Error>{grpc::Status::OK}; })
                         .or_else([](const data::Error& e) {
                             auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Failed to start game"};
                             return tl::expected<grpc::Status, data::Error>{status};
                         });
            return r.value();
        });
    }

    grpc::Status EngineServiceImpl::AddParticipant(grpc::ServerContext* context,
                                                   const board_game::AddParticipantRequest* request,
                                                   google::protobuf::Empty* response) {
        SPDLOG_DEBUG("AddParticipant");
        return pool.with_conn<grpc::Status>([request](pqxx::connection& conn) {
            data::RoomUUID room{std::string{request->room_uuid()}};
            data::ParticipantUUID participant{std::string{request->participant_uuid()}};
            auto role_expected = data::participant_role_from_string(request->participant_role());
            if (!role_expected.has_value()) {
                auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Failed to parse participant role"};
                return status;
            }
            data::ParticipantRole role = role_expected.value();

            auto r = data::add_participant(conn, room, participant, role)
                         .and_then([]() { return tl::expected<grpc::Status, data::Error>{grpc::Status::OK}; })
                         .or_else([](const data::Error& e) {
                             auto status =
                                 grpc::Status{grpc::StatusCode::INTERNAL, "Failed to add participant to room"};
                             return tl::expected<grpc::Status, data::Error>{status};
                         });
            return r.value();
        });
    }

} // namespace services

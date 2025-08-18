#include "engine_service.h"

#include <spdlog/spdlog.h>

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
        auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Not implemented"};
        return status;
    }

} // namespace services

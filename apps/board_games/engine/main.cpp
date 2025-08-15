#include "apps/board_games/engine/data/room.h"
#include "apps/board_games/engine/db/connection_pool.h"
#include "apps/board_games/engine/protocol/cli_service.grpc.pb.h"
#include "apps/board_games/engine/protocol/engine.grpc.pb.h"
#include <grpcpp/security/server_credentials.h>
#include <grpcpp/server_builder.h>
#include <pqxx/pqxx>
#include <spdlog/spdlog.h>

class CliServiceImpl final : public board_game::Cli::Service {
  public:
    CliServiceImpl(db::ConnectionPool& pool) : pool{pool} {}

    grpc::Status ListPlayingRooms(grpc::ServerContext* context, const google::protobuf::Empty* request,
                                  board_game::RoomList* response) override {
        SPDLOG_DEBUG("ListPlayingRooms");
        return pool.with_conn<grpc::Status>([response](pqxx::connection& conn) {
            auto r = data::get_playing_rooms(conn)
                         .and_then([response](auto playing_rooms) {
                             for (const auto& room_id : playing_rooms) {
                                 response->add_room_ids(room_id);
                             }
                             return std::expected<grpc::Status, data::Error>{grpc::Status::OK};
                         })
                         .or_else([](data::Error _e) {
                             auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Failed to retrieve playing rooms"};
                             return std::expected<grpc::Status, data::Error>{status};
                         });
            return r.value();
            // auto playing_rooms = data::get_playing_rooms(conn);
            // if (playing_rooms) {
            //     for (const auto& room_id : playing_rooms.value()) {
            //         response->add_room_ids(room_id);
            //     }
            //     return grpc::Status::OK;
            // } else {
            //     return grpc::Status{grpc::StatusCode::INTERNAL, "Failed to retrieve playing rooms"};
            // }
        });
    }

  private:
    db::ConnectionPool& pool;
};

class EngineServiceImpl final : public board_game::EngineService::Service {
  public:
    EngineServiceImpl(db::ConnectionPool& pool) : pool{pool} {}

    grpc::Status SubmitCommand(grpc::ServerContext* context, const board_game::CommandRequest* request,
                               board_game::CommandResponse* response) override {
        SPDLOG_DEBUG("SubmitCommand");
        pool.with_conn<void>([response](pqxx::connection& conn) { response->set_success(true); });
        return grpc::Status::OK;
    }

    grpc::Status CreateNewRoom(grpc::ServerContext* context, const board_game::NewRoomRequest* request,
                               google::protobuf::Empty* response) override {
        SPDLOG_DEBUG("CreateNewRoom");
        return pool.with_conn<grpc::Status>([request](pqxx::connection& conn) -> grpc::Status {
            auto r = data::insert_new_room(conn, data::RoomUUID{std::string{request->uuid()}}, request->name())
                         .and_then([]() { return std::expected<grpc::Status, data::Error>{grpc::Status::OK}; })
                         .or_else([](const data::Error& e) {
                             auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Failed to insert new room"};
                             return std::expected<grpc::Status, data::Error>{status};
                         });
            return r.value();
        });
    }

    grpc::Status StartGame(grpc::ServerContext* context, const board_game::StartGameRequest* request,
                           google::protobuf::Empty* response) override {
        SPDLOG_DEBUG("StartGame");
        auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Not implemented"};
        return status;
    }

  private:
    db::ConnectionPool& pool;
};

int main(int argc, char** argv) {
    spdlog::set_level(spdlog::level::debug); // TODO: Configurable via CLI and/or envvar
    spdlog::set_pattern("[%Y-%m-%d %H:%M:%S.%e][%^%8l%$][engine] %v (%@)");

    auto pool = db::ConnectionPool::from_env("BOARD_GAMES_ENGINE_", 4);

    // Working as a gRPC server
    std::string server_address = "[::]:50051";

    CliServiceImpl service{pool};
    EngineServiceImpl engine_service{pool};

    grpc::ServerBuilder builder;
    builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());
    builder.RegisterService(&service);
    builder.RegisterService(&engine_service);

    std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
    SPDLOG_INFO("Board games engine listening on {}", server_address);
    server->Wait();

    SPDLOG_INFO("Board games engine is finished");
    return 0;
}

#include "apps/board_games/engine/protocol/cli_service.grpc.pb.h"
#include "apps/board_games/engine/protocol/engine.grpc.pb.h"
#include "db/connection_pool.h"
#include "db/get_playing_room_ids.h"
#include <grpcpp/security/server_credentials.h>
#include <grpcpp/server_builder.h>
#include <iostream>
#include <pqxx/pqxx>

class CliServiceImpl final : public board_game::Cli::Service {
  public:
    CliServiceImpl(db::ConnectionPool& pool) : pool{pool} {}

    grpc::Status ListPlayingRooms(grpc::ServerContext* context, const google::protobuf::Empty* request,
                                  board_game::RoomList* response) override {
        std::cout << "[engine] ListPlayingRooms" << std::endl;
        pool.with_conn<void>([response](pqxx::connection& conn) {
            for (const auto& room_id : db::get_playing_room_ids(conn)) {
                response->add_room_ids(room_id);
            }
        });
        return grpc::Status::OK;
    }

  private:
    db::ConnectionPool& pool;
};

class EngineServiceImpl final : public board_game::EngineService::Service {
  public:
    EngineServiceImpl(db::ConnectionPool& pool) : pool{pool} {}

    grpc::Status SubmitCommand(grpc::ServerContext* context, const board_game::CommandRequest* request,
                               board_game::CommandResponse* response) override {
        std::cout << "[engine] ListPlayingRooms" << std::endl;
        pool.with_conn<void>([response](pqxx::connection& conn) { response->set_success(true); });
        return grpc::Status::OK;
    }

  private:
    db::ConnectionPool& pool;
};

int main(int argc, char** argv) {
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
    std::cout << "C++ server listening on " << server_address << std::endl;
    server->Wait();

    // PostgreSQL NOTIFY

    // pqxx::connection conn(connection_str.c_str());
    // pqxx::work txn(conn);

    return 0;
}

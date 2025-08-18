#include <grpcpp/security/server_credentials.h>
#include <grpcpp/server_builder.h>
#include <spdlog/spdlog.h>

#include "apps/board_games/engine/db/connection_pool.h"
#include "apps/board_games/engine/services/cli_service.h"
#include "apps/board_games/engine/services/engine_service.h"

int main(int argc, char** argv) {
    spdlog::set_level(spdlog::level::debug); // TODO: Configurable via CLI and/or envvar
    spdlog::set_pattern("[%Y-%m-%d %H:%M:%S.%e][%^%8l%$][engine] %v (%@)");

    auto pool = db::ConnectionPool::from_env("BOARD_GAMES_ENGINE_", 4);

    // Working as a gRPC server
    std::string server_address = "[::]:50051";

    services::CliServiceImpl service{pool};
    services::EngineServiceImpl engine_service{pool};

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

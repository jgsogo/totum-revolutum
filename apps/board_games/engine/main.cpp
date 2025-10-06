#include <grpcpp/security/server_credentials.h>
#include <grpcpp/server_builder.h>
#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/data/models/game_type.hpp"
#include "apps/board_games/engine/game_plugin.hpp"
#include "apps/board_games/engine/services/cli_service.h"
#include "apps/board_games/engine/services/engine_service.h"
#include "apps/board_games/games/tic_tac_toe/engine/tic_tac_toe.h"

int main(int argc, char** argv) {
    spdlog::set_level(spdlog::level::debug); // TODO: Configurable via CLI and/or envvar
    spdlog::set_pattern("[%Y-%m-%d %H:%M:%S.%e][%^%8l%$][engine] %v (%@)");

    // Collect the games
    engine::GamePluginsMap games;
    std::unique_ptr<engine::GamePluginBase> tic_tac_toe = std::make_unique<board_games::tic_tac_toe::TicTacToePlugin>();
    games.insert(std::make_pair(tic_tac_toe->slug(), std::move(tic_tac_toe)));

    auto pool_expected = utils::libpqxx::ConnectionPool::from_env("BOARD_GAMES_ENGINE_", 4);
    if (!pool_expected) {
        SPDLOG_ERROR("Failed to connect to the database: {}", pool_expected.error());
        return 1;
    }
    utils::libpqxx::ConnectionPool& pool = pool_expected.value();

    // TODO: Based on the registered games, enable/disable them in the DB
    if (!pool.with_conn<bool>([&games](pqxx::connection& conn) {
            std::vector<data::GameType> all_games;
            std::transform(games.begin(), games.end(), std::back_inserter(all_games),
                           [](const auto& pair) { return pair.first; });
            auto r = data::set_active_games(conn, all_games);
            return r.has_value();
        })) {
        SPDLOG_ERROR("Failed to update enabled games. There is nothing to here.");
        return 1;
    }

    // Working as a gRPC server
    std::string server_address = "[::]:50051";

    services::CliServiceImpl service{pool};
    services::EngineServiceImpl engine_service{pool, games};

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

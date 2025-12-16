#pragma once

#include "apps/board_games/engine/plugin_base/game_plugin_map.hpp"
#include "apps/board_games/engine/protocol/engine.grpc.pb.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

namespace services {

    class EngineServiceImpl final : public board_games::EngineService::Service {
      public:
        EngineServiceImpl(utils::libpqxx::ConnectionPool& pool, const engine::GamePluginsMap& games);

        grpc::Status CreateNewRoom(grpc::ServerContext* context, const board_games::NewRoomRequest* request,
                                   google::protobuf::Empty* response) override;

        grpc::Status StartGame(grpc::ServerContext* context, const board_games::StartGameRequest* request,
                               google::protobuf::Empty* response) override;

        grpc::Status GetOrCreateParticipant(grpc::ServerContext* context,
                                            const board_games::GetOrCreateParticipantRequest* request,
                                            google::protobuf::Empty* response) override;

        grpc::Status SendAction(grpc::ServerContext* context, const board_games::SendActionRequest* request,
                                google::protobuf::Empty* response) override;

      private:
        utils::libpqxx::ConnectionPool& pool;
        const engine::GamePluginsMap& _games;
    };

} // namespace services

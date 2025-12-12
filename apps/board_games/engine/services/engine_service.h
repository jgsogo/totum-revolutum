#pragma once

#include "apps/board_games/engine/game_plugin.hpp"
#include "apps/board_games/engine/protocol/engine.grpc.pb.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

namespace services {

    class EngineServiceImpl final : public board_game::EngineService::Service {
      public:
        EngineServiceImpl(utils::libpqxx::ConnectionPool& pool, const engine::GamePluginsMap& games);

        grpc::Status CreateNewRoom(grpc::ServerContext* context, const board_game::NewRoomRequest* request,
                                   google::protobuf::Empty* response) override;

        grpc::Status StartGame(grpc::ServerContext* context, const board_game::StartGameRequest* request,
                               google::protobuf::Empty* response) override;

        grpc::Status GetOrCreateParticipant(grpc::ServerContext* context,
                                            const board_game::GetOrCreateParticipantRequest* request,
                                            board_game::Participant* response) override;

        grpc::Status SendGameAction(grpc::ServerContext* context, const board_game::SendGameActionRequest* request,
                                    google::protobuf::Empty* response) override;

      private:
        utils::libpqxx::ConnectionPool& pool;
        const engine::GamePluginsMap& _games;
    };

} // namespace services

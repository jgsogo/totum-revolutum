#pragma once

#include "apps/board_games/engine/db/connection_pool.h"
#include "apps/board_games/engine/protocol/engine.grpc.pb.h"

namespace services {

    class EngineServiceImpl final : public board_game::EngineService::Service {
      public:
        EngineServiceImpl(db::ConnectionPool& pool);

        grpc::Status SubmitCommand(grpc::ServerContext* context, const board_game::CommandRequest* request,
                                   board_game::CommandResponse* response) override;

        grpc::Status CreateNewRoom(grpc::ServerContext* context, const board_game::NewRoomRequest* request,
                                   google::protobuf::Empty* response) override;

        grpc::Status StartGame(grpc::ServerContext* context, const board_game::StartGameRequest* request,
                               google::protobuf::Empty* response) override;

        grpc::Status GetOrCreateParticipant(grpc::ServerContext* context,
                                            const board_game::GetOrCreateParticipantRequest* request,
                                            board_game::Participant* response) override;

      private:
        db::ConnectionPool& pool;
    };

} // namespace services

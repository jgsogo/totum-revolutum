#pragma once

#include "apps/board_games/engine/protocol/cli_service.grpc.pb.h"
#include "libraries/utils/cpp/db/connection_pool.h"

namespace services {

    class CliServiceImpl final : public board_game::Cli::Service {
      public:
        CliServiceImpl(utils::db::ConnectionPool& pool);

        grpc::Status ListPlayingRooms(grpc::ServerContext* context, const google::protobuf::Empty* request,
                                      board_game::RoomList* response) override;

      private:
        utils::db::ConnectionPool& pool;
    };

} // namespace services

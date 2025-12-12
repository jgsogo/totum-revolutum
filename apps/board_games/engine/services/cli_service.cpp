#include "cli_service.h"

#include <spdlog/spdlog.h>

#include "apps/board_games/engine/data/room.h"

namespace services {

    CliServiceImpl::CliServiceImpl(utils::libpqxx::ConnectionPool& pool) : pool{pool} {}

    grpc::Status CliServiceImpl::ListPlayingRooms(grpc::ServerContext* context, const google::protobuf::Empty* request,
                                                  board_games::RoomList* response) {
        SPDLOG_DEBUG("ListPlayingRooms");
        return pool.with_conn<grpc::Status>([response](pqxx::connection& conn) {
            auto r = data::get_playing_rooms(conn)
                         .and_then([response](auto playing_rooms) {
                             for (const auto& room_id : playing_rooms) {
                                 response->add_room_ids(room_id);
                             }
                             return Expected<grpc::Status>{grpc::Status::OK};
                         })
                         .or_else([](const auto& e) {
                             auto status = grpc::Status{grpc::StatusCode::INTERNAL,
                                                        std::format("Failed to retrieve playing rooms", e)};
                             return Expected<grpc::Status>{status};
                         });
            return r.value();
        });
    }

} // namespace services

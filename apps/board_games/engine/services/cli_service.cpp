#include "cli_service.h"

#include <spdlog/spdlog.h>

#include "apps/board_games/engine/data/room.h"

namespace services {

    CliServiceImpl::CliServiceImpl(db::ConnectionPool& pool) : pool{pool} {}

    grpc::Status CliServiceImpl::ListPlayingRooms(grpc::ServerContext* context, const google::protobuf::Empty* request,
                                                  board_game::RoomList* response) {
        SPDLOG_DEBUG("ListPlayingRooms");
        return pool.with_conn<grpc::Status>([response](pqxx::connection& conn) {
            auto r = data::get_playing_rooms(conn)
                         .and_then([response](auto playing_rooms) {
                             for (const auto& room_id : playing_rooms) {
                                 response->add_room_ids(room_id);
                             }
                             return tl::expected<grpc::Status, data::Error>{grpc::Status::OK};
                         })
                         .or_else([](data::Error _e) {
                             auto status = grpc::Status{grpc::StatusCode::INTERNAL, "Failed to retrieve playing rooms"};
                             return tl::expected<grpc::Status, data::Error>{status};
                         });
            return r.value();
        });
    }

} // namespace services

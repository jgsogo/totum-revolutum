
#include <grpc/grpc.h>
#include <grpcpp/channel.h>
#include <grpcpp/create_channel.h>
#include <grpcpp/security/credentials.h>

#include "apps/board_games/engine/protocol/cli_service.grpc.pb.h"

int main(int argc, char** argv) {
    std::string server_address = "localhost:50051";
    std::cout << "Server Address: " << server_address << std::endl;

    std::shared_ptr<grpc::Channel> channel = grpc::CreateChannel(server_address, grpc::InsecureChannelCredentials());
    std::unique_ptr<board_game::Cli::Stub> stub_{board_game::Cli::NewStub(channel)};

    std::cout << "Server started." << std::endl;

    grpc::ClientContext context;
    board_game::RoomList room_list;
    grpc::Status r = stub_->ListPlayingRooms(&context, google::protobuf::Empty{}, &room_list);

    if (!r.ok()) {
        std::cout << "GetStatus rpc failed (" << r.error_code() << "): " << r.error_message() << std::endl;
        std::exit(EXIT_FAILURE);
        return -1;
    } else {
        std::cout << "Playing rooms: " << std::endl;
        for (const auto& room_id : room_list.room_ids()) {
            std::cout << " - " << room_id << std::endl;
        }
    }

    return 0;
}

#include <iostream>
#include <grpcpp/server_builder.h>
#include <grpcpp/security/server_credentials.h>
#include "apps/board-games/engine/protocol/cli_service.grpc.pb.h"

class CliServiceImpl final : public board_game::Cli::Service {
 public:

   grpc::Status GetStatus(grpc::ServerContext* context, const google::protobuf::Empty* request, board_game::Status* response) override {
    response->set_msg("I'm alive!");
    return grpc::Status::OK;
  }

};

int main(int argc, char** argv) {
    std::cout << "I'm the engine" << std::endl;

    std::string server_address = "[::]:50051";

    CliServiceImpl service;

    grpc::ServerBuilder builder;
    builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());
    builder.RegisterService(&service);
    std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
    std::cout << "C++ server listening on " << server_address << std::endl;
    server->Wait();

    return 0;
}

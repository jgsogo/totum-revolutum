// custom-main.cpp
#define CATCH_CONFIG_RUNNER
#include <catch2/catch_all.hpp>

#include <filesystem>
#include <iostream>

#include "test_config.hpp"

#include "apps/board_games/games/ticket_to_ride/maps/cpp/map_loader.h"

int main(int argc, char* argv[]) {
    Catch::Session session; // There must be exactly one instance

    using namespace Catch::Clara;

    std::filesystem::path textproto, protobin;
    // Extend Catch2's CLI
    auto cli = session.cli() | Opt(textproto, "textproto")["--textproto"]("Path to textproto file") |
               Opt(protobin, "protobin")["--protobin"]("Path to proto file (binary format)");

    session.cli(cli);

    // Parse CLI
    auto result = session.applyCommandLine(argc, argv);
    if (result != 0)
        return result; // Catch2 will print the error

    // One or the other, but not both
    assert(!textproto.empty() || !protobin.empty());
    assert(textproto.empty() || protobin.empty());

    // Populate TestConfig with the map_data, and run!
    auto& cfg = TestConfig::instance();
    if (!textproto.empty()) {
        auto map_data = board_games::ticket_to_ride::load_map_data(textproto);
        if (!map_data) {
            std::cerr << "Abort. " << map_data.error() << std::endl;
            return -1;
        }
        cfg.map_data = std::move(map_data.value());
    } else {
        auto map_data = board_games::ticket_to_ride::load_map_data_binary(protobin);
        if (!map_data) {
            std::cerr << "Abort. " << map_data.error() << std::endl;
            return -1;
        }
        cfg.map_data = std::move(map_data.value());
    }

    return session.run();
}

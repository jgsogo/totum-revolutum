#pragma once

#include "apps/board_games/games/ticket_to_ride/maps/map.pb.h"

struct TestConfig {
    board_games::ticket_to_ride::MapData map_data;

    static TestConfig& instance() {
        static TestConfig cfg;
        return cfg;
    }
};

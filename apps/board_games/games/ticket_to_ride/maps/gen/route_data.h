#pragma once

#include <string>
#include <vector>

#include "apps/board_games/games/ticket_to_ride/maps/map.pb.h"
#include "libraries/utils/cpp/math/geometry/2d/point.hpp"
#include "libraries/utils/cpp/svg/colors.h"

struct City {
    static City from(const board_games::ticket_to_ride::City& city);

    int32_t idx;
    std::string id;
    std::string name;

    utils::math::g2d::Point<int> pos;
};

struct RouteData {
    City start;
    City end;
    int32_t n_carriages;
    std::vector<svg::Color> colors;
    bool draw_ccw;
};

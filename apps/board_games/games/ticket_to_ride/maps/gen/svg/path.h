#pragma once

#include <optional>
#include <ostream>
#include <vector>

#include "libraries/utils/cpp/math/geometry/2d/point.hpp"

namespace svg {

    struct Path {
        std::optional<std::string> stroke;
        std::optional<int> stroke_width;
        std::optional<std::string> fill;

        std::vector<utils::math::g2d::Point<int>> segments;
    };

} // namespace svg

std::ostream& operator<<(std::ostream& os, const svg::Path& path);

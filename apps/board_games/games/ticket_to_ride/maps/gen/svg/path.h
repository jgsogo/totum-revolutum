#pragma once

#include <optional>
#include <ostream>
#include <vector>

#include "libraries/utils/cpp/math/geometry/2d/point.hpp"

#include "colors.h"
#include "svg.h"

namespace svg {

    struct Path : SVGElement {
        Path() = default;

        std::optional<Color> stroke;
        std::optional<int> stroke_width;
        std::optional<Color> fill;

        std::vector<utils::math::g2d::Point<int>> segments;

        std::ostream& write(std::ostream& os) const override final;
    };

} // namespace svg

std::ostream& operator<<(std::ostream& os, const svg::Path& path);

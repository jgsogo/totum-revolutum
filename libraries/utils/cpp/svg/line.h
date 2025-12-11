#pragma once

#include <optional>

#include "libraries/utils/cpp/math/geometry/2d/point.hpp"

#include "colors.h"
#include "svg.h"

namespace svg {

    struct Line : SVGElement {
        Line() = default;

        utils::math::g2d::Point<int> start;
        utils::math::g2d::Point<int> end;

        std::ostream& write(std::ostream& os) const override final;
    };

} // namespace svg

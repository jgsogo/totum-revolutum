#pragma once

#include <optional>

#include "libraries/utils/cpp/math/geometry/2d/point.hpp"

#include "colors.h"
#include "svg.h"

namespace svg {

    struct Rect : SVGElement {
        Rect() = default;

        utils::math::g2d::Point<int> size;

        std::ostream& write(std::ostream& os) const override final;
    };

} // namespace svg

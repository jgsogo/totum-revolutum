#pragma once

#include "libraries/utils/cpp/math/geometry/2d/point.hpp"

#include "svg.h"

namespace svg {

    struct Image : SVGElement {
        Image() = default;

        std::string href;
        utils::math::g2d::Point<int> size;

        std::ostream& write(std::ostream& os) const override final;
    };

} // namespace svg

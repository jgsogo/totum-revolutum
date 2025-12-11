#pragma once

#include <optional>

#include "libraries/utils/cpp/math/geometry/2d/circunference.hpp"
#include "libraries/utils/cpp/math/geometry/2d/point.hpp"

#include "colors.h"
#include "svg.h"

namespace svg {

    struct Circle : SVGElement {
        Circle() = default;

        utils::math::g2d::Point<int> center;
        int radius;

        std::ostream& write(std::ostream& os) const override final;

        template <typename T> static Circle from(const utils::math::g2d::Circunference<T>& circ) {
            Circle ret;
            ret.center = utils::math::g2d::Point<int>{
                .x = static_cast<int>(circ.center.x),
                .y = static_cast<int>(circ.center.y),
            };
            ret.radius = static_cast<int>(circ.radius);
            return ret;
        }
    };

} // namespace svg

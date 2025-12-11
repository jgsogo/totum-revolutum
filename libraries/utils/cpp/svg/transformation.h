#pragma once

#include <ostream>
#include <string>

#include "libraries/utils/cpp/math/geometry/2d/point.hpp"

namespace svg {

    struct Transformation {
        Transformation() = default;
        virtual ~Transformation() = default;

        virtual std::ostream& write(std::ostream& os) const = 0;
    };

    struct Translate : Transformation {
        explicit Translate(utils::math::g2d::Point<int> pos) : pos{pos} {};
        ~Translate() = default;

        std::ostream& write(std::ostream& os) const;

        utils::math::g2d::Point<int> pos;
    };

    struct Rotate : Transformation {
        explicit Rotate(float degrees) : degrees{degrees} {};
        ~Rotate() = default;

        std::ostream& write(std::ostream& os) const;

        float degrees;
    };

} // namespace svg

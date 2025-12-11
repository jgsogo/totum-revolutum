#pragma once

#include <cmath>

#include "point.hpp"

namespace utils::math::g2d {

    // Normalize angle to (-pi, pi]
    template <typename T> T normalize_ang(T angle_rads) {
        // 1. Shift the range to be [0, 2*PI) by adding PI and taking modulo 2*PI
        T normalized = fmod(angle_rads + M_PI, 2 * M_PI);

        // 2. Shift the range back to (-PI, PI] by subtracting PI
        return normalized - M_PI;
    }

    template <typename T> T atan2(const Point<T>& p) { return std::atan2(p.y, p.x); }

} // namespace utils::math::g2d

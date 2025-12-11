#pragma once

#include <optional>
#include <ostream>
#include <vector>

#include "libraries/utils/cpp/math/geometry/2d/circunference.hpp"
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

        template <typename T>
        static Path from(const utils::math::g2d::Circunference<T>& circ, const T& ang_start, const T& ang_end,
                         int n_segments) {
            Path path;
            for (int i = 0; i < (n_segments + 1); i++) {
                T ang = ang_start + i * (ang_end - ang_start) / n_segments;
                int x = static_cast<int>(circ.center.x + circ.radius * std::cos(ang));
                int y = static_cast<int>(circ.center.y + circ.radius * std::sin(ang));
                path.segments.emplace_back(utils::math::g2d::Point<int>{.x = x, .y = y});
            }
            return path;
        }
    };

} // namespace svg

std::ostream& operator<<(std::ostream& os, const svg::Path& path);

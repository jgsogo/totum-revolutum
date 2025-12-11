#pragma once

#include <optional>

#include "circunference.hpp"
#include "point.hpp"

namespace utils::math::g2d {

    // Computes the circunference that includes the given points, and the arch between those
    // points has the given length. If a solution exists, there will always be two solutions
    // with the same radius and centers at opposite sides.
    //
    // If the arc_length is smaller than the distance between the input points, the problem
    // doesn't have any solution
    template <typename T>
    std::optional<std::pair<Circunference<T>, Circunference<T>>> circle_for_arc(const Point<T>& p1, const Point<T>& p2,
                                                                                const T& arc_length) {
        const T dx = p2.x - p1.x;
        const T dy = p2.y - p1.y;
        const T chord = std::hypot(dx, dy);

        if (arc_length < chord) {
            // arc too short to connect both points
            return std::nullopt;
        }

        // Solve chord = 2 * (s/theta) * sin(theta/2)
        // Newton iteration for theta
        ////////////// const double s = arc_length;
        double theta = 2 * std::asin(chord / (2 * arc_length)); // reasonable initial guess

        for (int i = 0; i < 20; ++i) {
            double f = 2 * (arc_length / theta) * std::sin(theta / 2) - chord;
            double df = -2 * arc_length * std::sin(theta / 2) / (theta * theta) +
                        (arc_length / theta) * std::cos(theta / 2) / 2;

            double new_theta = theta - f / df;
            if (std::abs(new_theta - theta) < 1e-12)
                theta = new_theta;
            else
                theta = new_theta;
        }

        double R = arc_length / theta;
        double half_c = chord / 2.0;

        if (R < half_c)
            return std::nullopt; // numeric safety

        double h = std::sqrt(R * R - half_c * half_c);

        // Midpoint
        Point<T> M{.x = (p1.x + p2.x) * 0.5, .y = (p1.y + p2.y) * 0.5};

        // Perpendicular unit normal
        Point<T> n{.x = -dy / chord, .y = dx / chord};

        // Two centers
        Point<T> C1{.x = M.x + h * n.x, .y = M.y + h * n.y};
        Point<T> C2{.x = M.x - h * n.x, .y = M.y - h * n.y};

        return std::make_pair(Circunference<T>{.center = C1, .radius = R}, Circunference<T>{.center = C2, .radius = R});
    }
} // namespace utils::math::g2d

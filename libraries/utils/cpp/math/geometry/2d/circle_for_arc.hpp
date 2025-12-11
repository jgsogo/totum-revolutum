#pragma once

#include <optional>

#include "libraries/utils/cpp/math/newton_method.hpp"

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
        T theta = 2 * std::asin(chord / (2 * arc_length)); // reasonable initial guess
        {
            std::function<T(const T&)> f = [&arc_length, &chord](const T& theta) {
                return 2 * (arc_length / theta) * std::sin(theta / 2) - chord;
            };
            std::function<T(const T&)> f_prime = [&arc_length](const T& theta) {
                return -2 * arc_length * std::sin(theta / 2) / (theta * theta) +
                       (arc_length / theta) * std::cos(theta / 2) / 2;
            };

            theta = utils::math::newton_method<T, T>(theta, f, f_prime, 20);
        }

        T radius = arc_length / theta;
        T half_chord = chord / 2.0;

        if (radius < half_chord)
            return std::nullopt; // numeric safety

        T h = std::sqrt(radius * radius - half_chord * half_chord);

        // Midpoint
        Point<T> M{.x = (p1.x + p2.x) * 0.5, .y = (p1.y + p2.y) * 0.5};

        // Perpendicular unit normal
        Point<T> n{.x = -dy / chord, .y = dx / chord};

        // Two centers
        Point<T> C1{.x = M.x + h * n.x, .y = M.y + h * n.y};
        Point<T> C2{.x = M.x - h * n.x, .y = M.y - h * n.y};

        return std::make_pair(Circunference<T>{.center = C1, .radius = radius},
                              Circunference<T>{.center = C2, .radius = radius});
    }
} // namespace utils::math::g2d

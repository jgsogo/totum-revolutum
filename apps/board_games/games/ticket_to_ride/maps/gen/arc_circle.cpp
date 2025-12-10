#include "arc_circle.h"

#include <cmath>

std::optional<CircleSolution> arc_circle(Vec2 A, Vec2 B, double arc_length) {
    double dx = B.x - A.x;
    double dy = B.y - A.y;
    double c = std::hypot(dx, dy);

    if (arc_length < c) {
        // arc too short to connect both points
        return std::nullopt;
    }

    // Solve c = 2 * (s/theta) * sin(theta/2)
    // Newton iteration for theta
    const double s = arc_length;
    double theta = 2 * std::asin(c / (2 * s)); // reasonable initial guess

    for (int i = 0; i < 20; ++i) {
        double f = 2 * (s / theta) * std::sin(theta / 2) - c;
        double df = -2 * s * std::sin(theta / 2) / (theta * theta) + (s / theta) * std::cos(theta / 2) / 2;

        double new_theta = theta - f / df;
        if (std::abs(new_theta - theta) < 1e-12)
            theta = new_theta;
        else
            theta = new_theta;
    }

    double R = s / theta;
    double half_c = c / 2.0;

    if (R < half_c)
        return std::nullopt; // numeric safety

    double h = std::sqrt(R * R - half_c * half_c);

    // Midpoint
    Vec2 M{(A.x + B.x) * 0.5, (A.y + B.y) * 0.5};

    // Perpendicular unit normal
    Vec2 n{-dy / c, dx / c};

    // Two centers
    Vec2 C1{M.x + h * n.x, M.y + h * n.y};
    Vec2 C2{M.x - h * n.x, M.y - h * n.y};

    return CircleSolution{.radius = R, .centers = {C1, C2}};
}

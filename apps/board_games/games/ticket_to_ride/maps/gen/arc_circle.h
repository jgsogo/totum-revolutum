#pragma once

#include <array>
#include <optional>

struct Vec2 {
    double x, y;
};

struct CircleSolution {
    double radius;
    std::array<Vec2, 2> centers; // two possible centers
};

std::optional<CircleSolution> arc_circle(Vec2 A, Vec2 B, double arc_length);

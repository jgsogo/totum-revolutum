#pragma once

#include <cmath>

namespace utils::math::g2d {

    template <typename T = float> struct Point {
        T x;
        T y;

        template <typename T1> operator Point<T1>() const {
            return Point<T1>{.x = static_cast<T1>(x), .y = static_cast<T1>(y)};
        }

        Point operator-() const { return Point{.x = -x, .y = -y}; }

        T hypot() const { return std::hypot(x, y); }
    };

    template <typename T> Point<T> operator+(const Point<T>& p1, const Point<T>& p2) {
        return Point<T>{.x = p1.x + p2.x, .y = p1.y + p2.y};
    }

    template <typename T> Point<T> operator-(const Point<T>& p1, const Point<T>& p2) {
        return Point<T>{.x = p1.x - p2.x, .y = p1.y - p2.y};
    }

    template <typename T> Point<T> operator*(const T& lhs, const Point<T>& p) {
        return Point<T>{.x = lhs * p.x, .y = lhs * p.y};
    }

    template <typename T> Point<T> operator/(const Point<T>& p, const T& rhs) {
        return Point<T>{.x = p.x / rhs, .y = p.y / rhs};
    }

} // namespace utils::math::g2d

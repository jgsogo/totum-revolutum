#pragma once

#include "point.hpp"
#include <format>

namespace utils::math::g2d {

    template <typename T = float> struct Circunference {
        Point<T> center;
        T radius;

        //! Creates a new `Circunference` from the given values.
        //!
        //! It will raise if the radius is not a positive number.
        static Circunference from(Point<T>&& center, T&& radius) noexcept(false) {
            if (radius <= 0) {
                throw std::runtime_error(std::format("Provided radius '{}' is invalid, expected a value > 0", radius));
            }
            return Circunference{.center = std::move(center), .radius = std::move(radius)};
        }
    };

} // namespace utils::math::g2d

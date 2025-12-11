#pragma once

#include <ostream>

namespace svg {

    enum class Color {
        GREY,
        BLUE,
        RED,
        GREEN,
        YELLOW,
        BLACK,
        WHITE,
        ORANGE,
        HOTPINK,

        // special colors
        TRANSPARENT,
    };

}

std::ostream& operator<<(std::ostream&, const svg::Color&);

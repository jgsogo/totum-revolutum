#include "path.h"

std::ostream& operator<<(std::ostream& os, const svg::Path& path) {
    os << "<path";
    if (path.stroke) {
        os << " stroke='" << path.stroke.value() << "'";
    }
    if (path.stroke_width) {
        os << " stroke-width='" << path.stroke_width.value() << "'";
    }
    if (path.fill) {
        os << " fill='" << path.fill.value() << "'";
    }

    // the segments
    os << " d='M";
    for (const auto& segment : path.segments) {
        os << segment.x << " " << segment.y << " ";
    }
    os << "'";

    os << "/>\n";
    return os;
}

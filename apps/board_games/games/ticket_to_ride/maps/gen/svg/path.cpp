#include "path.h"

namespace svg {

    std::ostream& Path::write(std::ostream& os) const {
        os << "<path";
        if (stroke) {
            os << " stroke='" << stroke.value() << "'";
        }
        if (stroke_width) {
            os << " stroke-width='" << stroke_width.value() << "'";
        }
        if (fill) {
            os << " fill='" << fill.value() << "'";
        }

        // the segments
        os << " d='M";
        for (const auto& segment : segments) {
            os << segment.x << " " << segment.y << " ";
        }
        os << "'";

        os << "/>\n";
        return os;
    }

} // namespace svg

std::ostream& operator<<(std::ostream& os, const svg::Path& path) { return path.write(os); }

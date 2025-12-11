#include "path.h"

namespace svg {

    std::ostream& Path::write(std::ostream& os) const {
        os << "<path";
        this->_write(os);

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

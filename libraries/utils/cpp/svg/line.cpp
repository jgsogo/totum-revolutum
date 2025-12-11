#include "line.h"

namespace svg {

    std::ostream& Line::write(std::ostream& os) const {
        os << "<line";
        this->_write(os);

        os << " x1='" << start.x << "' y1='" << start.y << "'";
        os << " x2='" << end.x << "' y2='" << end.y << "'";

        os << "/>\n";
        return os;
    }

} // namespace svg

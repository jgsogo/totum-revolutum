#include "circle.h"

namespace svg {

    std::ostream& Circle::write(std::ostream& os) const {
        os << "<circle";
        this->_write(os);

        os << " cx='" << center.x << "'";
        os << " cy='" << center.y << "'";
        os << " r='" << radius << "'";

        os << "/>\n";
        return os;
    }

} // namespace svg

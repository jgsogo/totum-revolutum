#include "rect.h"

namespace svg {

    std::ostream& Rect::write(std::ostream& os) const {
        os << "<rect";
        this->_write(os);

        os << " width='" << size.x << "'";
        os << " height='" << size.y << "'";

        os << "/>\n";
        return os;
    }

} // namespace svg

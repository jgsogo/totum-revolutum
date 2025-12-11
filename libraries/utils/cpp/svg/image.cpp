#include "image.h"

namespace svg {

    std::ostream& Image::write(std::ostream& os) const {
        os << "<image";
        this->_write(os);
        os << " href='" << href << "' width='" << size.x << "' height='" << size.y << "' />\n";
        return os;
    }

} // namespace svg

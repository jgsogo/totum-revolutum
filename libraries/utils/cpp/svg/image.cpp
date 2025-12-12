#include "image.h"

namespace svg {

    std::ostream& Image::write(std::ostream& os) const {
        os << "<image";
        this->_write(os);
        os << " href='" << href << "'";
        if (size.has_value()) {
            os << " width='" << size.value().x << "' height='" << size.value().y << "'";
        }
        os << "/>\n";
        return os;
    }

} // namespace svg

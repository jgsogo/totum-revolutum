#include "transformation.h"

namespace svg {

    std::ostream& Translate::write(std::ostream& os) const {
        os << " translate(" << pos.x << ", " << pos.y << ")";
        return os;
    }

    std::ostream& Rotate::write(std::ostream& os) const {
        os << " rotate(" << degrees << ")";
        return os;
    }

} // namespace svg

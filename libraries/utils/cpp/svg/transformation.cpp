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

    std::ostream& Scale::write(std::ostream& os) const {
        os << " scale(" << scale << ")";
        return os;
    }

} // namespace svg

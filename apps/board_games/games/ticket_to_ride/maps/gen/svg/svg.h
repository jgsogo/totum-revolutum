#pragma once

#include <ostream>
#include <string>

namespace svg {

    struct SVGElement {
        virtual std::ostream& write(std::ostream& os) const = 0;
    };

} // namespace svg

#include "svg.h"

namespace svg {

    std::ostream& SVGElement::_write(std::ostream& os) const {
        if (stroke) {
            os << " stroke='" << stroke.value() << "'";
        }
        if (stroke_width) {
            os << " stroke-width='" << stroke_width.value() << "'";
        }
        if (fill) {
            os << " fill='" << fill.value() << "'";
        }
        if (!transformation.empty()) {
            os << " transform='";
            for (const auto& t : transformation) {
                t->write(os);
            }
            os << "'";
        }

        return os;
    }

    std::ostream& SVGGroup::write(std::ostream& os) const {
        os << "<g>\n";
        if (id) {
            os << " id='" << id.value() << "'";
        }

        for (const auto& elem : elements) {
            os << "  ";
            elem->write(os);
        }

        os << "</g>\n";
        return os;
    }
} // namespace svg

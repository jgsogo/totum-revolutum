#include "svg.h"

namespace svg {

    std::ostream& SVGElement::_write(std::ostream& os) const {
        if (id) {
            os << " id='" << id.value() << "'";
        }
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

    SVGUse::SVGUse(std::string href) : href{href} {}

    std::ostream& SVGUse::write(std::ostream& os) const {
        os << "<use";
        this->_write(os);
        os << " href='" << href << "' x='" << pos.x << "' y='" << pos.y << "'";
        os << "/>\n";
        return os;
    }

    std::ostream& SVGVector::write(std::ostream& os) const {
        for (const auto& elem : elements) {
            os << "  ";
            elem->write(os);
        }
        return os;
    }

    std::ostream& SVGGroup::write(std::ostream& os) const {
        os << "<g";
        if (id) {
            os << " id='" << id.value() << "'";
        }
        os << ">\n";

        this->SVGVector::write(os);

        os << "</g>\n";
        return os;
    }

    std::ostream& SVGDefs::write(std::ostream& os) const {
        os << "<defs>\n";

        this->SVGVector::write(os);

        os << "</defs>\n";
        return os;
    }

    std::ostream& SVGDoc::write(std::ostream& os) const {
        os << "<svg";
        os << " width='" << size.x << "' height='" << size.y << "'";
        os << " xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink'";
        os << ">\n";

        this->SVGVector::write(os);

        os << "</svg>\n";
        return os;
    }

} // namespace svg

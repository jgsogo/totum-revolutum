#pragma once

#include <memory>
#include <ostream>
#include <string>
#include <vector>

#include "colors.h"
#include "transformation.h"

namespace svg {

    struct SVGElement {
        SVGElement() = default;
        virtual ~SVGElement() = default;

        SVGElement(const SVGElement&) = delete;
        SVGElement& operator=(const SVGElement&) = delete;

        SVGElement(SVGElement&&) noexcept = default;
        SVGElement& operator=(SVGElement&&) noexcept = default;

        std::optional<Color> stroke;
        std::optional<float> stroke_width;
        std::optional<Color> fill;
        std::vector<std::unique_ptr<Transformation>> transformation;

        std::ostream& _write(std::ostream& os) const;
        virtual std::ostream& write(std::ostream& os) const = 0;
    };

    struct SVGGroup : SVGElement {
        SVGGroup() = default;

        std::ostream& write(std::ostream& os) const;

        template <typename T, typename... Args> T& add(Args&&... args) {
            auto ptr = std::make_unique<T>(std::forward<Args>(args)...);
            T& ref = *ptr;
            elements.push_back(std::move(ptr));
            return ref;
        }

        template <typename T, typename... Args> T& add_from(Args&&... args) {
            auto elem = T::from(std::forward<Args>(args)...);
            auto ptr = std::make_unique<T>(std::move(elem));
            T& ref = *ptr;
            elements.push_back(std::move(ptr));
            return ref;
        }

        std::optional<std::string> id;
        std::vector<std::unique_ptr<SVGElement>> elements;
    };

} // namespace svg

inline std::ostream& operator<<(std::ostream& os, const svg::SVGElement& elem) { return elem.write(os); }

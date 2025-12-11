#include "colors.h"

std::ostream& operator<<(std::ostream& os, const svg::Color& color) {
    switch (color) {
    case svg::Color::GREY:
        os << "grey";
        break;
    case svg::Color::BLUE:
        os << "blue";
        break;
    case svg::Color::RED:
        os << "red";
        break;
    case svg::Color::GREEN:
        os << "green";
        break;
    case svg::Color::YELLOW:
        os << "yellow";
        break;
    case svg::Color::BLACK:
        os << "black";
        break;
    case svg::Color::WHITE:
        os << "white";
        break;
    case svg::Color::ORANGE:
        os << "orange";
        break;
    case svg::Color::HOTPINK:
        os << "hotpink";
        break;
    case svg::Color::TRANSPARENT:
        os << "transparent";
        break;
    }
    return os;
}

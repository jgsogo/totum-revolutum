#include "route_data.h"

namespace {

    std::string city_id(std::string name) {
        std::transform(name.begin(), name.end(), name.begin(), [](unsigned char c) -> unsigned char {
            if (c == ' ')
                return '-';
            else
                return std::tolower(c);
        });
        return name;
    }

} // namespace

City City::from(const board_games::ticket_to_ride::City& proto_city) {
    City city;
    city.idx = proto_city.id();
    city.name = proto_city.name();
    city.id = city_id(proto_city.name());
    city.pos = utils::math::g2d::Point<int>{.x = proto_city.pos_x(), .y = proto_city.pos_y()};
    return city;
}

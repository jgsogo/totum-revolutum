#include "usa.h"

namespace board_games::ticket_to_ride {

    namespace {

        void populate_cities(MapDefinition& map) {
            map.add_cities("City1");
            map.add_cities("City2");
        }

        void populate_routes(MapDefinition& map) {
            RouteDefinition* route = map.add_routes();
            route->set_id(0);
            route->set_city_a(0);
            route->set_city_b(1);
        }

    } // namespace

    void populate_usa_map(MapDefinition& map) {
        map.set_name("USA");

        populate_cities(map);
        populate_routes(map);
    }
} // namespace board_games::ticket_to_ride

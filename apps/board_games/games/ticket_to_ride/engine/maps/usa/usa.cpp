#include "usa.h"

namespace board_games::ticket_to_ride {

    static const std::vector<std::string> USA_CITIES = {
        "Vancouver", // 0
        "Seattle",   // 1
    };

    struct RouteRow {
        uint32_t a, b;
        bool is_grey;
        Color color;
        uint32_t length;
        bool second;
    };

    static const std::vector<RouteRow> USA_ROUTES = {

        // --- Pacific Northwest ---
        {0, 1, true, Color::RED, 1, false}, // Vancouver–Seattle
        {0, 1, true, Color::RED, 1, true},

    };

    namespace {

        void populate_cities(MapDefinition& map) {

            for (const auto& c : USA_CITIES) {
                map.add_cities(c);
            }
        }

        void populate_routes(MapDefinition& map) {
            uint32_t id = 0;
            for (const auto& row : USA_ROUTES) {
                RouteDefinition* r = map.add_routes();
                r->set_id(id++);
                r->set_city_a(row.a);
                r->set_city_b(row.b);

                if (row.is_grey)
                    r->set_any(true);
                else
                    r->set_color(row.color);

                r->set_length(row.length);
                r->set_is_tunnel(false);
                r->set_is_second_route(row.second);
            }
        }

    } // namespace

    void populate_usa_map(MapDefinition& map) {
        map.set_name("USA");

        populate_cities(map);
        populate_routes(map);
    }
} // namespace board_games::ticket_to_ride

#include <catch2/catch_test_macros.hpp>

#include "test_config.hpp"

TEST_CASE("Validate routes") {

    auto& cfg = TestConfig::instance();
    const auto& routes = cfg.map_data.routes();
    const int n_routes = cfg.map_data.routes_size();
    const int n_cities = cfg.map_data.cities_size();
    REQUIRE(n_routes > 0);

    SECTION("routes") {

        int index = 0;
        std::unordered_set<std::string> single_routes; // for detecting duplicates
        std::unordered_map<std::string, int> double_route_count;

        for (const auto& r : routes) {
            CAPTURE(index++, r.city1(), r.city2(), r.color(), r.length(), r.double_route());

            // --- Validate city references ---
            int a = r.city1();
            int b = r.city2();

            REQUIRE(a >= 0);
            REQUIRE(a < n_cities);
            REQUIRE(b >= 0);
            REQUIRE(b < n_cities);
            REQUIRE(a != b);

            // --- Validate length ---
            REQUIRE(r.length() > 0);
            REQUIRE(r.length() <= 6);

            // --- Validate color enum ---
            // REQUIRE(Route_Color_IsValid(r.color()));

            // --- Check for duplicate routes ---
            // normalize city order: smaller first
            int ca = std::min(a, b);
            int cb = std::max(a, b);
            std::string key = std::to_string(ca) + "-" + std::to_string(cb);

            if (r.double_route()) {
                // --- Track double-route pairs ---
                double_route_count[key]++;
            } else {
                REQUIRE(single_routes.insert(key).second);
            }
        }

        // ----------------------------------------------------
        // 3. Validate double route logic: exactly 2 per pair
        // ----------------------------------------------------
        for (const auto& [pair, count] : double_route_count) {
            CAPTURE(pair);
            REQUIRE(count == 2);
        }
    }
}

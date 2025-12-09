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
        std::unordered_set<std::string> route_keys; // for detecting duplicates
        std::unordered_map<std::string, int> double_route_count;

        for (const auto& r : routes) {
            CAPTURE(index++, r.city1(), r.city2(), r.color(), r.length(), r.double_route());

            // REQUIRE(c.id() >= 0);
            // REQUIRE(c.id() < n_cities);
            // REQUIRE(!id_seen[c.id()]);
            // id_seen[c.id()] = true;

            // REQUIRE(!c.name().empty());
            // REQUIRE(name_seen.insert(c.name()).second);
        }
    }
}

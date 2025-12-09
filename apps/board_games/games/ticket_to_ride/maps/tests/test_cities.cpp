#include <catch2/catch_test_macros.hpp>

#include "test_config.hpp"

TEST_CASE("Validate cities") {

    auto& cfg = TestConfig::instance();
    const auto& cities = cfg.map_data.cities();
    const int n_cities = cfg.map_data.cities_size();
    REQUIRE(n_cities > 0);

    SECTION("cities") {

        int index = 0;
        std::unordered_set<std::string> name_seen;
        std::vector<bool> id_seen(n_cities, false);

        for (const auto& c : cities) {
            CAPTURE(index++, c.id(), c.name());

            REQUIRE(c.id() >= 0);
            REQUIRE(c.id() < n_cities);
            REQUIRE(!id_seen[c.id()]);
            id_seen[c.id()] = true;

            REQUIRE(!c.name().empty());
            REQUIRE(name_seen.insert(c.name()).second);
        }
    }
}

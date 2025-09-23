#include <catch2/catch_test_macros.hpp>

#include "libraries/utils/cpp/catch2/capture_spdlog.hpp"
#include "libraries/utils/cpp/integral_type.hpp"

using namespace utils;

TEST_CASE("Test integral_type") {

    SECTION("Test algebraic operations") {
        utils::IntegralType<class Testing, uint8_t> value{uint8_t{42}};
        REQUIRE(value == 42);

        utils::IntegralType<class Testing, uint8_t> other{uint8_t{23}};
        REQUIRE(value > other);
        REQUIRE(value != other);
        REQUIRE(other <= value);
    }

    SECTION("Test formating") {
        utils::IntegralType<class Testing, uint8_t> value{uint8_t{42}};
        REQUIRE(std::format("{}", value) == "42");
        REQUIRE(fmt::format("{}", value) == "42");

        std::stringstream os;
        os << value;
        REQUIRE(os.str() == "42");
    }

    SECTION("Test libpqxx") {
        // TODO
    }
}

TEST_CASE_METHOD(utils::catch2::CaptureSpdlogFixture, "Test capture spdlog fixture") {
    SECTION("Test spdlog") {
        utils::IntegralType<class Testing, uint8_t> value{uint8_t{42}};

        SPDLOG_INFO("my log {} value", value);
        REQUIRE(oss.str().find("[info]") != std::string::npos);
        REQUIRE(oss.str().find("my log 42 value") != std::string::npos);
    }
}

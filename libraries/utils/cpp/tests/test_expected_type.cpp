#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>

#include "libraries/utils/cpp/catch2/capture_spdlog.hpp"
#include "libraries/utils/cpp/expected_type.hpp"

using namespace utils;

// TEST_CASE("Test expected_type") {

//     // SECTION("Test algebraic operations") {
//     //     utils::IntegralType<class Testing, uint8_t> value{uint8_t{42}};
//     //     REQUIRE(value == 42);

//     //     utils::IntegralType<class Testing, uint8_t> other{uint8_t{23}};
//     //     REQUIRE(value > other);
//     //     REQUIRE(value != other);
//     //     REQUIRE(other <= value);
//     // }

//     // SECTION("Test formating") {
//     //     utils::IntegralType<class Testing, uint8_t> value{uint8_t{42}};
//     //     REQUIRE(std::format("{}", value) == "42");
//     //     REQUIRE(fmt::format("{}", value) == "42");

//     //     std::stringstream os;
//     //     os << value;
//     //     REQUIRE(os.str() == "42");
//     // }

//     // SECTION("Test libpqxx") {
//     //     // TODO
//     // }
// }

TEST_CASE("Test std::format output") {

    SECTION("Test std::format -- error - NotImplemented") {
        ExpectedType<std::string, int> not_implemented = tl::unexpected{NotImplemented{"sorry"}};

        CHECK_THAT(std::format("{}", not_implemented.error()), Catch::Matchers::Equals("NotImplemented: sorry"));
    }

    SECTION("Test std::format -- error - other") {
        ExpectedType<std::string, int> not_implemented = tl::unexpected{42};

        CHECK_THAT(std::format("{}", not_implemented.error()), Catch::Matchers::Equals("42"));
    }
}

TEST_CASE_METHOD(utils::catch2::CaptureSpdlogFixture, "Test spdlog output") {
    SECTION("Test spdlog -- error - NotImplemented") {
        ExpectedType<std::string, int> not_implemented = tl::unexpected{NotImplemented{"sorry"}};

        SPDLOG_INFO("spdlog output: {}", not_implemented.error());
        CHECK_THAT(oss.str(), Catch::Matchers::ContainsSubstring("NotImplemented: sorry"));
    }

    SECTION("Test spdlog -- error - other") {
        ExpectedType<std::string, int> not_implemented = tl::unexpected{42};

        SPDLOG_INFO("spdlog output: {}", not_implemented.error());
        CHECK_THAT(oss.str(), Catch::Matchers::ContainsSubstring("spdlog output: 42"));
    }
}

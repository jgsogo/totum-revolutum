#include <catch2/catch_test_macros.hpp>

#include "libraries/utils/cpp/db/catch2/capture_spdlog.hpp"
#include "libraries/utils/cpp/string_type.hpp"

using namespace utils;

TEST_CASE("Test string_type") {

    SECTION("Test algebraic operations") {
        utils::StringType<class Testing> value{std::string{"str1"}};
        REQUIRE(value == "str1");

        utils::StringType<class Testing> other{std::string{"str0"}};
        REQUIRE(value > other);
        REQUIRE(value != other);
        REQUIRE(other <= value);
    }

    SECTION("Test formating") {
        utils::StringType<class Testing> value{std::string{"str1"}};
        REQUIRE(std::format("{}", value) == "str1");
        REQUIRE(fmt::format("{}", value) == "str1");

        std::stringstream os;
        os << value;
        REQUIRE(os.str() == "str1");
    }

    SECTION("Test libpqxx") {
        // TODO
    }
}

TEST_CASE_METHOD(utils::catch2::CaptureSpdlogFixture, "Test capture spdlog fixture") {
    SECTION("Test spdlog") {
        utils::StringType<class Testing> value{std::string{"str1"}};

        SPDLOG_INFO("my log {} value", value);
        REQUIRE(oss.str().find("[info]") != std::string::npos);
        REQUIRE(oss.str().find("my log str1 value") != std::string::npos);
    }
}

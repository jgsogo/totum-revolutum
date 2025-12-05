#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>

#include "libraries/utils/cpp/catch2/capture_spdlog.hpp"
#include "libraries/utils/cpp/expected_type/error_type.hpp"

TEST_CASE("Test ErrorType") {
    using ErrorType = utils::ErrorType<int, std::string>;

    ErrorType error_int{42};
    ErrorType error_not_implemented{utils::NotImplemented{"sorry"}};
    ErrorType error_str{"str error"};

    SECTION("std::format") {
        CHECK_THAT(std::format("{}", error_int), Catch::Matchers::Equals("42"));
        CHECK_THAT(std::format("{}", error_not_implemented), Catch::Matchers::Equals("NotImplemented: sorry"));
        CHECK_THAT(std::format("{}", error_str), Catch::Matchers::Equals("str error"));
    }

    SECTION("construct from subset") {
        utils::ErrorType<int> subset{42};
        ErrorType e1{std::move(subset)};
        ErrorType e2 = static_cast<ErrorType>(std::move(subset));
    }

    SECTION("check equal with operator==") {
        // We can compare if the underlying types provides a implementation for ==
        REQUIRE(error_int == 42);
        REQUIRE(error_str == std::string{"str error"});
        // ...otherwise we cannot compare
        // REQUIRE(error_not_implemented == utils::NotImplemented{"sorry"});

        // If the value contained is different, they are not equal
        REQUIRE(error_int != 23);

        // They are not equal for different types
        REQUIRE(error_int != std::string{"str error"});
    }
}

TEST_CASE_METHOD(utils::catch2::CaptureSpdlogFixture, "Test ErrorType - spdlog") {
    using ErrorType = utils::ErrorType<int, std::string>;

    ErrorType error_int{42};
    ErrorType error_not_implemented{utils::NotImplemented{"sorry"}};
    ErrorType error_str{"str error"};

    SECTION("error_int") {
        SPDLOG_INFO("spdlog output: {}", error_int);
        CHECK_THAT(oss.str(), Catch::Matchers::ContainsSubstring("42"));
    }

    SECTION("error_not_implemented") {
        SPDLOG_INFO("spdlog output: {}", error_not_implemented);
        CHECK_THAT(oss.str(), Catch::Matchers::ContainsSubstring("NotImplemented: sorry"));
    }

    SECTION("error_str") {
        SPDLOG_INFO("spdlog output: {}", error_str);
        CHECK_THAT(oss.str(), Catch::Matchers::ContainsSubstring("str error"));
    }
}

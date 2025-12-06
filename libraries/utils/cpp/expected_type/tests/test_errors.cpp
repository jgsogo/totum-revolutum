
#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>

#include "libraries/utils/cpp/catch2/capture_spdlog.hpp"
#include "libraries/utils/cpp/expected_type/errors.hpp"

TEST_CASE("Test BaseError type") {
    utils::NotImplemented not_implemented{"sorry"};
    utils::errors::BaseError<"MyCustomError"> other_error{"sorry too"};

    SECTION("std::format") {
        CHECK_THAT(std::format("{}", not_implemented), Catch::Matchers::Equals("NotImplemented: sorry"));
        CHECK_THAT(std::format("{}", other_error), Catch::Matchers::Equals("MyCustomError: sorry too"));
    }
}

TEST_CASE_METHOD(utils::catch2::CaptureSpdlogFixture, "Test BaseError type - spdlog") {
    utils::NotImplemented not_implemented{"sorry"};
    utils::errors::BaseError<"MyCustomError"> other_error{"sorry too"};

    SECTION("spdlog") {
        SPDLOG_INFO("spdlog output: {}", not_implemented);
        CHECK_THAT(oss.str(), Catch::Matchers::ContainsSubstring("NotImplemented: sorry"));
    }

    SECTION("MyCustomError") {
        SPDLOG_INFO("spdlog output: {}", other_error);
        CHECK_THAT(oss.str(), Catch::Matchers::ContainsSubstring("spdlog output: MyCustomError: sorry too"));
    }
}

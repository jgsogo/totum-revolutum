
#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>

#include "libraries/utils/cpp/catch2/capture_spdlog.hpp"
#include "libraries/utils/cpp/expected_type/errors.hpp"

TEST_CASE("Test std::format output") {

    SECTION("NotImplemented") {
        utils::NotImplemented e{"sorry"};
        CHECK_THAT(std::format("{}", e), Catch::Matchers::Equals("NotImplemented: sorry"));
    }

    SECTION("MyCustomError") {
        utils::errors::BaseError<"MyCustomError"> e{"sorry too"};
        CHECK_THAT(std::format("{}", e), Catch::Matchers::Equals("MyCustomError: sorry too"));
    }
}

TEST_CASE_METHOD(utils::catch2::CaptureSpdlogFixture, "Test spdlog output") {
    SECTION("NotImplemented") {
        utils::NotImplemented e{"sorry"};
        SPDLOG_INFO("spdlog output: {}", e);
        CHECK_THAT(oss.str(), Catch::Matchers::ContainsSubstring("NotImplemented: sorry"));
    }

    SECTION("MyCustomError") {
        utils::errors::BaseError<"MyCustomError"> e{"sorry too"};
        SPDLOG_INFO("spdlog output: {}", e);
        CHECK_THAT(oss.str(), Catch::Matchers::ContainsSubstring("spdlog output: MyCustomError: sorry too"));
    }
}

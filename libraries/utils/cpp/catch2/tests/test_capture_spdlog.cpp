#include <catch2/catch_test_macros.hpp>

#include "libraries/utils/cpp/catch2/capture_spdlog.hpp"

using namespace utils::catch2;

TEST_CASE_METHOD(CaptureSpdlogFixture, "Test capture spdlog fixture") {
    REQUIRE(oss.str() == "");

    SPDLOG_INFO("log string");
    REQUIRE(oss.str().find("[info]") != std::string::npos);
    REQUIRE(oss.str().find("log string") != std::string::npos);

    reset();
    REQUIRE(oss.str() == "");
}

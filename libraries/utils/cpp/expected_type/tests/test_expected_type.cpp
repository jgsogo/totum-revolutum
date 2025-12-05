#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>

#include "libraries/utils/cpp/catch2/capture_spdlog.hpp"
#include "libraries/utils/cpp/expected_type/expected_type.hpp"

TEST_CASE("Test ExpectedType") {
    using ExpectedType = utils::ExpectedType<std::string, int, std::string>;

    ExpectedType success{"success"};
    ExpectedType error_int{std::unexpected(42)};
    ExpectedType error_str{std::unexpected("error")};
    ExpectedType error_not_implemented{std::unexpected(utils::NotImplemented{"todo"})};

    SECTION("and_then") {
        ExpectedType r = success.and_then([](const std::string& str) { return ExpectedType{"other success"}; });
        REQUIRE(r.has_value());
        REQUIRE(r.value() == "other success");
    }
}

#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>

#include "libraries/utils/cpp/catch2/capture_spdlog.hpp"
#include "libraries/utils/cpp/expected_type/expected_type.hpp"

TEST_CASE("Test ExpectedType") {
    using ExpectedType = utils::ExpectedType<std::string, int, std::string>;

    ExpectedType success{"success"};
    ExpectedType error_int{tl::unexpected(42)};
    ExpectedType error_str{tl::unexpected("error")};
    ExpectedType error_not_implemented{tl::unexpected(utils::NotImplemented{"todo"})};

    SECTION("and_then") {
        // We can return the same success type
        ExpectedType r = success.and_then([](const std::string& str) { return ExpectedType{"other success"}; });
        REQUIRE(r.has_value());
        REQUIRE(r.value() == "other success");

        // We can also change the success type
        auto r2 =
            success.and_then([](const std::string& str) { return utils::ExpectedType<int, int, std::string>{0}; });
        REQUIRE(r2.has_value());
        REQUIRE(r2.value() == 0);

        // ...but we cannot change the ErrorType
        // auto r3 = success.and_then([](const std::string& str) { return utils::ExpectedType<int>{0}; });
    }

    SECTION("or_else") {
        // We can return the same error type (with different value)
        ExpectedType r = error_int.or_else([](auto& e) { return ExpectedType(tl::unexpected(-1)); });
        REQUIRE(!r.has_value());
        REQUIRE(r.error() == -1);

        // We can also change the error type
        auto r2 = error_int.or_else([](auto& e) { return ExpectedType(tl::unexpected("other error")); });
        REQUIRE(!r2.has_value());
        REQUIRE(r2.error() == std::string{"other error"});

        // and we can return from a subset of errors, and they are implicitly converted to the superset
        ExpectedType r3 = error_int.or_else([](auto& e) {
            return utils::ExpectedType<std::string, std::string>{tl::unexpected(tl::unexpected("other error"))};
        });
        REQUIRE(!r3.has_value());
        REQUIRE(r3.error() == std::string{"other error"});

        // .... but we cannot return from a different set of errors
        // error_int.or_else([](auto& e) { return utils::ExpectedType<std::string, float>{tl::unexpected(1.2f)}; });
    }

    SECTION("transform") {
        // We can return the same success type
        ExpectedType r = success.transform([](const std::string& str) { return std::string{"other success"}; });
        REQUIRE(r.has_value());
        REQUIRE(r.value() == "other success");

        // We can also change the success type
        auto r2 = success.transform([](const std::string& str) { return 0; });
        REQUIRE(r2.has_value());
        REQUIRE(r2.value() == 0);
    }

    SECTION("transform_error") {
        // We can return the same error type (with different value)
        ExpectedType r = error_int.transform_error([](auto& e) { return -1; });
        REQUIRE(!r.has_value());
        REQUIRE(r.error() == -1);

        // We can also change the error type
        auto r2 = error_int.transform_error([](auto& e) { return "other error"; });
        REQUIRE(!r2.has_value());
        REQUIRE(r2.error() == std::string{"other error"});
    }
}

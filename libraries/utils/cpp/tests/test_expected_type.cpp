#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>

#include "libraries/utils/cpp/catch2/capture_spdlog.hpp"
#include "libraries/utils/cpp/expected_type.hpp"

using namespace utils;

TEST_CASE("Test ctor") {

    SECTION("Test regular ctor") {
        REQUIRE(ExpectedType<std::string, int>{"success"}.has_value());
        REQUIRE(!ExpectedType<std::string, int>{NotImplemented{"error"}}.has_value());
        REQUIRE(!ExpectedType<std::string, int>{42}.has_value());
    }

    SECTION("Ambiguous type") {
        ExpectedType<std::string, int, std::string> t{"what"};
        REQUIRE(t.has_value());

        ExpectedType<std::string, int, std::string> t2 = tl::unexpected{"what"};
        REQUIRE(!t2.has_value());
    }
}

TEST_CASE("Test ctor from subset of errors") {

    SECTION("Construct from the errors") {
        using Subset = ExpectedType<void, int>;
        using Superset = ExpectedType<void, int, std::string>;

        Subset subset{42};
        REQUIRE(!subset.has_value());

        Superset superset{std::move(subset.error())};
        REQUIRE(!superset.has_value());
    }

    SECTION("Forward constructor") {
        using Subset = ExpectedType<void, int>;
        using Superset = ExpectedType<void, int, std::string>;

        Subset subset{42};
        REQUIRE(!subset.has_value());

        Superset superset = std::move(subset);
        REQUIRE(!superset.has_value());
    }
}

TEST_CASE("Test and_then") {
    SECTION("and_then -- with value") {
        auto and_then_f = [](int) { return ExpectedType<std::string>{"success"}; };

        ExpectedType<int> s{};
        auto r = s.and_then(and_then_f);
        REQUIRE(r.has_value());
        REQUIRE(r.value() == "success");
    }

    SECTION("and_then -- void value") {
        auto and_then_f = []() { return ExpectedType<std::string>{"success"}; };

        ExpectedType<void> s{};
        auto r = s.and_then(and_then_f);
        REQUIRE(r.has_value());
        REQUIRE(r.value() == "success");
    }
}

TEST_CASE("Test or_else") {
    // SECTION("or_else -- with value") {
    //     auto or_else_f = [](auto&& e){
    //         return ExpectedType<int>{42};
    //     };

    //     ExpectedType<int> s{NotImplemented{"sorry"}};
    //     ExpectedType<int> r = s.or_else(or_else_f);
    //     // IMHO, this is a missing feature in tl::expected: 'or_else' returns a tl::expected
    //     //       instance, instead of using 'auto' and returning 'ExpectedType' here.

    //     REQUIRE(r.has_value());
    //     REQUIRE(r.value() == 42);
    // }
}

TEST_CASE("Test transform") {
    SECTION("transform -- with value") {
        auto transform_f = [](int) { return ExpectedType<std::string>{"success"}; };

        ExpectedType<int> s{};
        auto r = s.transform(transform_f);
        REQUIRE(r.has_value());
        REQUIRE(r.value() == "success");
    }

    SECTION("transform -- void value") {
        auto transform_f = []() { return ExpectedType<std::string>{"success"}; };

        ExpectedType<void> s{};
        auto r = s.transform(transform_f);
        REQUIRE(r.has_value());
        REQUIRE(r.value() == "success");
    }
}

TEST_CASE("Test transform_error") {
    // SECTION("transform_error -- with value") {
    //     auto transform_error_f = [](std::variant<NotImplemented>& e){
    //         // return ExpectedType<int>{42};
    //         return "error-string";
    //     };

    //     ExpectedType<int> s{NotImplemented{"sorry"}};
    //     ExpectedType<int> r = s.transform_error(transform_error_f);
    //     // IMHO, this is a missing feature in tl::expected: 'transform_error' returns a tl::expected
    //     //       instance, instead of using 'auto' and returning 'ExpectedType' here.

    //     REQUIRE(r.has_value());
    //     REQUIRE(r.value() == 42);
    // }
}

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

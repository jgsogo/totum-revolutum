#include <catch2/catch_test_macros.hpp>

#include "libraries/utils/cpp/string_literal.hpp"

using namespace utils;

template <StringLiteral lit> struct Something {
    constexpr static auto literal = lit;
    constexpr static auto contents = lit.value;
};

TEST_CASE("Test string_literal") {

    Something<"the string"> something;

    SECTION("Test formatting") { REQUIRE(something.literal == "the string"); }

    SECTION("Test formating") {
        REQUIRE(std::format("{}", something.literal) == "the string");
        REQUIRE(fmt::format("{}", something.literal) == "the string");

        std::stringstream os;
        os << something.literal;
        REQUIRE(os.str() == "the string");
    }
}

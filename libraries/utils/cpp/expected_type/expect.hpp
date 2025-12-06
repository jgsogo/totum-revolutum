#pragma once

// #include <utility>
// #include <type_traits>

#include "./expected_type.hpp"

namespace utils::_impl {

    // Detect if a type is an expected
    template <class T> struct is_expected : std::false_type {};

    template <class T, class... Errs> struct is_expected<ExpectedType<T, Errs...>> : std::true_type {};

} // namespace utils::_impl

// EXPECT(expr) -> extracts T or propagates E
//
// Usage:
//     auto x = EXPECT(do_something());
//     auto y = EXPECT(other());
//
// Requires the function to return ExpectedType<T,E>.
#define EXPECT(expr)                                                                                                   \
    ({                                                                                                                 \
        auto&& _exp = (expr);                                                                                          \
                                                                                                                       \
        using _ExpT = std::decay_t<decltype(_exp)>;                                                                    \
        static_assert(utils::_impl::is_expected<_ExpT>::value,                                                         \
                      "EXPECT() requires an Expected<T, Errs...> expression");                                         \
                                                                                                                       \
        using _ValT = std::decay_t<decltype(_exp.value())>;                                                            \
        static_assert(std::is_move_constructible_v<_ValT>, "EXPECT(expr): value type must be move-constructible");     \
        static_assert(std::is_nothrow_move_constructible_v<_ValT>,                                                     \
                      "EXPECT(expr): value type must be nothrow-move-constructible");                                  \
                                                                                                                       \
        if (!_exp)                                                                                                     \
            return tl::unexpected(_exp.error());                                                                       \
        std::forward<decltype(_exp.value())>(_exp.value());                                                            \
    })

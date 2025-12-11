#pragma once

#include <assert.h>
#include <functional>

namespace utils::math {

    // Applies Newton's method to approximate `X` where f(X) = 0
    //
    //  - `initial_value`: initial guess estimate
    //  - `f`: function to evaluate
    //  - `f_prime`: derivative of the `f` function
    //  - `continue_f`: a function that will be called to check if the current
    //                  value is a good enough approach.
    template <typename X, typename Y>
    X newton_method(const X& initial_value, std::function<Y(const X&)> f, std::function<Y(const X&)> f_prime,
                    std::function<bool(const X& prev_value, const X& new_value)> continue_f) {
        X prev_value = initial_value;
        X new_value = initial_value;
        do {
            prev_value = new_value;
            new_value = prev_value - (f(prev_value) / f_prime(prev_value));
        } while (continue_f(prev_value, new_value));

        return new_value;
    }

    // Applies Newton's method to approximate `X` where f(X) = 0
    //
    //  - `initial_value`: initial guess estimate
    //  - `f`: function to evaluate
    //  - `f_prime`: derivative of the `f` function
    //  - `max_iterations`: an upper limit for the max number of iterations. Note that
    //                      a value equal or less than zero will translate into no limit
    template <typename X, typename Y>
    X newton_method(const X& initial_value, std::function<Y(const X&)> f, std::function<Y(const X&)> f_prime,
                    int max_iterations) {
        assert(max_iterations > 0 && "max_iterations has to be greater than zero");
        return newton_method<X, Y>(initial_value, f, f_prime,
                                   [&max_iterations](const X&, const X&) { return --max_iterations != 0; });
    }

} // namespace utils::math

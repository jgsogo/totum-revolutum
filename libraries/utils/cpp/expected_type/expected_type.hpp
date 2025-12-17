#pragma once

#include <optional>

// Eventhough we are using C++23, MacOS doesn't offer full support for C++23,
// and std::expected is missing in the system libc++ library... we need to
// upgrade to some newer MacOS version.
#include <tl/expected.hpp>

#include "./error_type.hpp"

namespace utils {

    template <typename T, typename... Errs> using ExpectedType = tl::expected<T, ErrorType<Errs...>>;

    namespace expected {

        //! Converts from ExpectedType<T, Errs...> to std::optional<T>.
        //!
        //! It consumes the ExpectedType input argument, discarding the error if any
        template <typename T, typename... Errs> std::optional<T> ok(ExpectedType<T, Errs...>&& exp) {
            if (exp.has_value()) {
                return {std::move(exp.value())};
            }
            return std::nullopt;
        }

        //! Converts from ExpectedType<T, Errs...> to std::optional<Err>.
        //!
        //! It consumes the ExpectedType input argument, discarding the success value if any
        template <typename Err, typename T, typename... Errs>
            requires utils::concepts::alternative_in_pack<Err, NotImplemented, Errs...>
        std::optional<Err> err(ExpectedType<T, Errs...>&& exp) {
            if (!exp.has_value()) {
                return std::visit(
                    [](auto&& arg) -> std::optional<Err> {
                        using TT = std::decay_t<decltype(arg)>;
                        if constexpr (std::is_same_v<TT, Err>) {
                            return std::optional<Err>{std::move(arg)};
                        } else {

                            return std::nullopt;
                        }
                    },
                    std::move(exp.error()));
            }
            return std::nullopt;
        }

        //! Transforms an std::optional<T> into ExpectedType<T, Errs...>
        template <typename T, typename... Errs, typename Err>
        ExpectedType<T, Errs..., Err> ok_or(std::optional<T>&& opt, Err&& err) {
            if (opt) {
                return ExpectedType<T, Errs..., Err>{std::move(*opt)};
            }
            return tl::unexpected(std::forward<Err>(err));
        }

    } // namespace expected
} // namespace utils

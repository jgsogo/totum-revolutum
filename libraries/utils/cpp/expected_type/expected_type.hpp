#pragma once

// Eventhough we are using C++23, MacOS doesn't offer full support for C++23,
// and std::expected is missing in the system libc++ library... we need to
// upgrade to some newer MacOS version.
#include <tl/expected.hpp>

#include "./error_type.hpp"

namespace utils {

    template <typename T, typename... Errs> using ExpectedType = tl::expected<T, ErrorType<Errs...>>;
}

#pragma once

#include <expected>

#include "./error_type.hpp"

namespace utils {

    template <typename T, typename... Errs> using ExpectedType = std::expected<T, ErrorType<Errs...>>;
}

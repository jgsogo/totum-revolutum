#pragma once

#include "libraries/utils/cpp/integral_type.hpp"
#include <cstdint>

namespace utils::db {
    using Id = utils::IntegralType<class DatabaseId, uint64_t>;
}

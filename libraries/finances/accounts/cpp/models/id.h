#pragma once

#include "libraries/utils/cpp/integral_type.hpp"

namespace finances::accounts::models {
    using Id = utils::IntegralType<class DatabaseId, uint64_t>;
}

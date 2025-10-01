#pragma once

#include "libraries/utils/cpp/libpqxx/datatypes/numeric.h"

namespace finances::accounts::models {
    using Amount = utils::libpqxx::Numeric<14, 4>;
}

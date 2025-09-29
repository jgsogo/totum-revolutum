#pragma once

#include "tl/expected.hpp"
#include <QString>

#include "libraries/finances/accounts/cpp/models/types/amount.h"

namespace utils {
    tl::expected<finances::accounts::models::Amount, std::string> qstring_to_amount(QString&& input);
}

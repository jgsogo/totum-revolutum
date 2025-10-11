#pragma once

#include <QString>
#include <tl/expected.hpp>

#include "libraries/finances/accounts/cpp/models/types/amount.h"

namespace utils {
    tl::expected<finances::accounts::models::Amount, std::string> qstring_to_amount(QString&& input);
}

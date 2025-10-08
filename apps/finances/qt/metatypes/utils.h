#pragma once

#include "tl/expected.hpp"
#include <QString>

#include "libraries/finances/accounts/cpp/models/types/amount.h"
#include "libraries/finances/accounts/cpp/models/types/ccy.h"

namespace utils {
    tl::expected<finances::accounts::models::Amount, std::string>
    qstring_to_amount(const QString& input, finances::accounts::models::Ccy ccy);
}

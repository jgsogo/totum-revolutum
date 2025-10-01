#pragma once

#include "libraries/finances/accounts/cpp/models/types/amount.h"

namespace finances::investments::models {

    struct NumerableAmount {
        finances::accounts::models::Amount quantity;
        finances::accounts::models::Amount unit_value;

        finances::accounts::models::Amount amount() const;
    };

} // namespace finances::investments::models

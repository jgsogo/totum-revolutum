#pragma once

#include "libraries/finances/accounts/cpp/models/movement.h"

namespace finances::investments::models {
    struct MovementNumerable {
        finances::accounts::models::Movement movement;

        utils::db::Id id;
        finances::accounts::models::Amount quantity;
        finances::accounts::models::Amount unit_value;
    };

} // namespace finances::investments::models

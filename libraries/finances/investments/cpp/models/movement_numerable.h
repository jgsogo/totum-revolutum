#pragma once

#include "libraries/finances/accounts/cpp/models/movement.h"

namespace finances::investments::models {
    class MovementNumerableManager;

    struct MovementNumerable : finances::accounts::models::Movement {
        using Manager = MovementNumerableManager;

        finances::accounts::models::Id id;
        finances::accounts::models::Amount quantity;
        finances::accounts::models::Amount unit_value;
    };

    class MovementNumerableManager : public finances::accounts::models::ModelManager<MovementNumerable> {
      public:
        tl::expected<std::vector<MovementNumerable>, finances::accounts::models::Error>
        all(finances::accounts::models::Id account_id);
    };
} // namespace finances::investments::models

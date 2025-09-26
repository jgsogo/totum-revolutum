#pragma once

#include <optional>
#include <string>

#include "types/id.h"

namespace finances::accounts::models {

    struct Transaction {
        Id id;
        std::string name;
        std::optional<std::string> description;
    };
} // namespace finances::accounts::models

#pragma once

#include <optional>
#include <string>

#include "types/id.h"

namespace finances::accounts::models {
    struct AccountType {
        Id id;
        std::string name;
        std::optional<std::string> description;
        bool is_abstract;
        std::optional<std::string> unique_name;
    };
} // namespace finances::accounts::models

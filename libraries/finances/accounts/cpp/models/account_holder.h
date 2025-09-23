#pragma once

#include <optional>
#include <string>

#include "types/id.h"

namespace finances::accounts::models {
    struct AccountHolder {
        Id id;
        std::string name;
        bool is_company;
        std::optional<std::string> photo;
    };
} // namespace finances::accounts::models

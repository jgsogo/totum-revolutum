#pragma once

#include <optional>
#include <string>

#include "country.h"
#include "id.h"

namespace finances::accounts::models {

    struct Custodian {
        Id id;
        std::string name;
        std::optional<std::string> description;
        Country country;
        std::optional<std::string> photo;
    };
} // namespace finances::accounts::models

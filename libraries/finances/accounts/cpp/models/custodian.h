#pragma once

#include <optional>
#include <string>

#include "types/country.h"

namespace finances::accounts::models {

    struct Custodian {
        utils::db::Id id;
        std::string name;
        std::optional<std::string> description;
        Country country;
        std::optional<std::string> photo;
    };
} // namespace finances::accounts::models

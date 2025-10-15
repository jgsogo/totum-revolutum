#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/orm/id.h"

namespace finances::accounts::models {

    struct Transaction {
        utils::db::Id id;
        std::string name;
        std::optional<std::string> description;
    };
} // namespace finances::accounts::models

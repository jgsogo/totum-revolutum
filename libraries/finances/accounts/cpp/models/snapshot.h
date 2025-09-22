#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/date.h"

#include "id.h"

namespace finances::accounts::models {

    struct Snapshot {
        Id id;
        decltype(AccountType::id) account_id;
        utils::libpqxx::Date date_value;
        float amount;
    };
} // namespace finances::accounts::models

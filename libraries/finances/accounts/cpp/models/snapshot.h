#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"

#include "types/amount.h"
#include "types/id.h"

namespace finances::accounts::models {

    struct Snapshot {
        Id id;
        decltype(AccountType::id) account_id;
        utils::libpqxx::Date date_value;
        Amount amount;
    };
} // namespace finances::accounts::models

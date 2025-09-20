#pragma once

#include <optional>
#include <string>

#include "ccy.h"
#include "id.h"
#include "model_manager.hpp"

namespace finances::accounts::models {

    struct Account {
        Id id;
        std::string name;
        std::optional<std::string> description;
        std::optional<std::string> identifier;
        Ccy ccy;
        // pub open: chrono::NaiveDate,
        // pub close: Option<chrono::NaiveDate>,
        Id type_id;
        Id custodian_id;
        bool is_numerable;
    };

    using AccountManager = ModelManager<Account>;

    template <> tl::expected<std::vector<Account>, Error> AccountManager::all();

} // namespace finances::accounts::models

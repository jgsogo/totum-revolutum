#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"

#include "custodian.h"
#include "hierarchy_tree.h"
#include "model_manager.hpp"
#include "types/ccy.h"
#include "types/id.h"

namespace finances::accounts::models {

    struct Account {
        Id id;
        std::string name;
        std::optional<std::string> description;
        std::optional<std::string> identifier;
        Ccy ccy;
        utils::libpqxx::Date open;
        std::optional<utils::libpqxx::Date> close;
        std::pair<decltype(AccountType::id), decltype(AccountType::name)> type;
        std::pair<decltype(Custodian::id), decltype(Custodian::name)> custodian;
        bool is_numerable;
    };

    using AccountManager = ModelManager<Account>;
    template <> tl::expected<std::vector<Account>, Error> ModelManager<Account>::all();

} // namespace finances::accounts::models

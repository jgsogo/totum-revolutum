#pragma once

#include <optional>
#include <string>

#include "model_manager.hpp"
#include "types/id.h"

namespace finances::accounts::models {
    class AccountHolderManager;

    struct AccountHolder {
        using Manager = AccountHolderManager;

        Id id;
        std::string name;
        bool is_company;
        std::optional<std::string> photo;
    };

    struct AccountHolderRole {
        bool owns_money;
    };

    class AccountHolderManager : public ModelManager<AccountHolder> {
      public:
        tl::expected<std::vector<std::pair<AccountHolder, AccountHolderRole>>, Error> all(Id account_id);
    };

    template <> tl::expected<AccountHolder, Error> ModelManager<AccountHolder>::get(Id id);
} // namespace finances::accounts::models

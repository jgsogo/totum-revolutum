#pragma once

#include <optional>
#include <string>

#include "model_manager.hpp"
#include "types/id.h"

namespace finances::accounts::models {

    template <typename Tag> class HierarchyTreeManager;

    template <typename Tag> struct HierarchyTree {
        using Manager = HierarchyTreeManager<HierarchyTree<Tag>>;

        Id id;
        std::string name;
        std::optional<std::string> description;
        bool is_abstract;
        std::optional<std::string> unique_name;
    };

    using AccountType = HierarchyTree<class AccountTypeTag>;
    using MovementType = HierarchyTree<class MovementTypeTag>;

    template <typename THierarchyTree> class HierarchyTreeManager : public ModelManager<THierarchyTree> {
      public:
        tl::expected<std::vector<std::string>, Error> breadcrumb(Id id);
    };

    template <> tl::expected<std::vector<std::string>, Error> HierarchyTreeManager<AccountType>::breadcrumb(Id id);

    template <> tl::expected<std::vector<std::string>, Error> HierarchyTreeManager<MovementType>::breadcrumb(Id id);

} // namespace finances::accounts::models

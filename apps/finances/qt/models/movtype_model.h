#pragma once

#include "libraries/finances/accounts/cpp/models/hierarchy_tree.h"

/// A model wrapping finances::accounts::movel::MovementType with some additional data
struct MovementTypeModel {
    utils::db::Id id;

    finances::accounts::models::MovementType movtype;
    std::string breadcrumb;
};

namespace utils::db {

    template <> ExpectedType<std::vector<MovementTypeModel>, DatabaseError> ModelManager<MovementTypeModel>::all();

    template <>
    ExpectedType<MovementTypeModel, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<MovementTypeModel>::get(const ModelData<MovementTypeModel>::Id&);
} // namespace utils::db

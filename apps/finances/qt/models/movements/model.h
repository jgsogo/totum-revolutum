#pragma once

#include "libraries/finances/accounts/cpp/models/movement.h"

#include "libraries/utils/cpp/libpqxx/orm/manager.h"

struct MovementModel {
    utils::db::Id id;

    finances::accounts::models::Movement movement;
    std::string movtype_breadcrumb;
};

namespace utils::db {

    template <> ExpectedType<std::vector<MovementModel>, DatabaseError> ModelManager<MovementModel>::all();

} // namespace utils::db

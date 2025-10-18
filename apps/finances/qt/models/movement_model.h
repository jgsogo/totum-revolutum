#pragma once

#include "libraries/finances/accounts/cpp/models/movement.h"

#include "libraries/utils/cpp/libpqxx/orm/manager.h"

/// A model wrapping finances::accounts::movel::Movement with some additional data
struct MovementModel {
    utils::db::Id id;

    finances::accounts::models::Movement movement; // std::variant with MovementNumerable and MovementDividend
    std::string movtype_breadcrumb;
};

namespace utils::db {

    template <>
    template <>
    ExpectedType<std::vector<MovementModel>, DatabaseError>
    ModelManager<MovementModel>::filter_by_fk<finances::accounts::models::Account>(
        const finances::accounts::models::Account& account);

    template <> ExpectedType<std::vector<MovementModel>, DatabaseError> ModelManager<MovementModel>::all();

    template <>
    ExpectedType<MovementModel, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<MovementModel>::get(const ModelData<MovementModel>::Id&);

} // namespace utils::db

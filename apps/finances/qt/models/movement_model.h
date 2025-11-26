#pragma once

#include "libraries/utils/cpp/libpqxx/orm/manager.h"

#include "libraries/finances/accounts/cpp/models/movement.h"
#include "libraries/finances/investments/cpp/models/movement_dividend.h"
#include "libraries/finances/investments/cpp/models/movement_numerable.h"

/// A model wrapping finances::accounts::movel::Movement with some additional data
struct MovementModel {
    utils::db::Id id;

    std::variant<finances::accounts::models::Movement, finances::investments::models::MovementNumerable,
                 finances::investments::models::MovementDividend>
        movement;
    std::string movtype_breadcrumb;

    const finances::accounts::models::Movement& as_movement() const;
};

namespace utils::db {

    template <>
    template <>
    ExpectedType<std::vector<MovementModel>, DatabaseError>
    ModelManager<MovementModel>::filter_by_fk<finances::accounts::models::Account>(
        const finances::accounts::models::Account& account);

    template <>
    template <>
    ExpectedType<std::vector<MovementModel>, DatabaseError>
    ModelManager<MovementModel>::filter_by_fk<finances::accounts::models::Transaction>(
        const finances::accounts::models::Transaction&);

    template <> ExpectedType<std::vector<MovementModel>, DatabaseError> ModelManager<MovementModel>::all();

    template <>
    ExpectedType<MovementModel, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<MovementModel>::get(const ModelData<MovementModel>::Id&);

    template <> Id ModelManager<MovementModel>::_create(pqxx::work&, const MovementModel&);

} // namespace utils::db

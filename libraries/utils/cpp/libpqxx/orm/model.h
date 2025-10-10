#pragma once

#include <pqxx/pqxx>

#include "id.h"

namespace utils::db {

    template <typename TModel> struct Model : TModel {
        using Id = decltype(TModel::id);
        using TModel::table_name;

        static TModel parse(const pqxx::row);
    };
} // namespace utils::db

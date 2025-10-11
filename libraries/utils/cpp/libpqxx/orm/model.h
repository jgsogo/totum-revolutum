#pragma once

#include <pqxx/pqxx>
#include <vector>

#include "libraries/utils/cpp/expected_type.hpp"
#include "libraries/utils/cpp/type_name.hpp"

#include "errors.h"
#include "id.h"

namespace utils::db {

    template <typename TModel> struct Model : TModel {
        using Id = decltype(TModel::id);
        // using TModel::table_name;
        static constexpr std::string_view name = utils::type_name<TModel>();

        static std::vector<TModel> get_all(pqxx::work&);
        static ExpectedType<TModel, ErrorNotFound, ErrorMultipleFound> get(pqxx::work&, const Id&);
    };
} // namespace utils::db

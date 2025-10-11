#pragma once

#include "id.h"
#include "libraries/utils/cpp/type_name.hpp"
#include <pqxx/pqxx>
#include <vector>

namespace utils::db {

    template <typename TModel> struct Model : TModel {
        using Id = decltype(TModel::id);
        // using TModel::table_name;
        static constexpr std::string_view name = utils::type_name<TModel>();

        static std::vector<TModel> get_all(pqxx::work&);
    };
} // namespace utils::db

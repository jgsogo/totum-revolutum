#pragma once

#include <pqxx/pqxx>
#include <vector>

#include "libraries/utils/cpp/expected_type.hpp"
#include "libraries/utils/cpp/type_name.hpp"

#include "errors.h"
#include "id.h"

namespace utils::db {

    template <typename TModel> class ModelManager;

    template <typename TModel> struct ModelData {
        using Model = TModel;
        using Manager = ModelManager<TModel>;
        using Id = decltype(TModel::id);
        static constexpr std::string_view name = utils::type_name<TModel>();

        // static constexpr std::string_view table_name; // to be provided by each model
    };
} // namespace utils::db

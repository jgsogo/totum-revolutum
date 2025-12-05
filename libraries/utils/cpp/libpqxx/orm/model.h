#pragma once

#include <pqxx/pqxx>
#include <vector>

#include "libraries/utils/cpp/expected_type/expected_type.hpp"
#include "libraries/utils/cpp/type_name.hpp"

#include "errors.h"
#include "id.h"

namespace utils::db {

    template <typename TModel> class ModelManager;

    template <typename TModel> struct ModelData {
        static constexpr std::string_view name = utils::type_name<TModel>();

        using Model = TModel;
        using Manager = ModelManager<TModel>;
        using Id = utils::db::Id;

        // Check 'TModel::id' field
        static_assert(std::is_same_v<decltype(TModel::id), utils::db::Id>,
                      "'TModel::id' must be of type 'utils::db::Id' to use this framework");
    };
} // namespace utils::db

#pragma once

#include <QVariant>
#include <Qt>

#include "libraries/utils/cpp/type_name.hpp"
#include <magic_enum/magic_enum.hpp>

namespace utils::qt::models {

    template <typename TModel, typename TColumn, enum Qt::ItemDataRole> struct DataDispatcher {
        static QVariant data(const TModel&, TColumn) { return QVariant{}; }
    };

    template <typename TModel, typename TColumn> struct DataDispatcher<TModel, TColumn, Qt::DisplayRole> {

        static QVariant data(const TModel&, TColumn column) {
            return QVariant{QString{"%1 - %2"}.arg(utils::type_name<TModel>()).arg(magic_enum::enum_name(column))};
        }
    };

} // namespace utils::qt::models

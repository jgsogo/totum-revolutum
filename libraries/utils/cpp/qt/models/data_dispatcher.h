#pragma once

#include <Qt>

namespace utils::qt::models {

    template <typename TModel, typename TColumn, enum Qt::ItemDataRole> struct DataDispatcher {
        static QVariant data(const TModel&, TColumn) { return QVariant{}; }

        template <typename T> static QVariant data(const TModel&, TColumn, const T&) { return QVariant{}; }
    };

    template <typename TModel, typename TColumn> struct DataDispatcher<TModel, TColumn, Qt::DisplayRole> {

        static QVariant data(const TModel&, TColumn column) {
            return QVariant{QString{"%1 - %2"}.arg(utils::type_name<TModel>()).arg(magic_enum::enum_name(column))};
        }

        template <typename T> static QVariant data(const TModel& model, TColumn column, const T&) {
            return data(model, column);
        }
    };

} // namespace utils::qt::models

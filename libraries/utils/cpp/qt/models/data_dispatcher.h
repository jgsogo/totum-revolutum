#pragma once

namespace utils::qt::models {

    template <class TModel, typename TColumn> class TableModel;

    template <typename TModel, typename TColumn, enum Qt::ItemDataRole> struct DataDispatcher {
        static QVariant data(const TableModel<TModel, TColumn>&, const TModel&, TColumn) { return QVariant{}; }
    };

    template <typename TModel, typename TColumn> struct DataDispatcher<TModel, TColumn, Qt::DisplayRole> {
        static QVariant data(const TableModel<TModel, TColumn>&, const TModel&, TColumn column) {
            return QVariant{QString{"%1 - %2"}.arg(utils::type_name<TModel>()).arg(magic_enum::enum_name(column))};
        }
    };

} // namespace utils::qt::models

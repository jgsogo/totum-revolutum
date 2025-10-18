#pragma once

#include <magic_enum/magic_enum.hpp>
#include <spdlog/spdlog.h>

#include <QAbstractTableModel>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"
#include "libraries/utils/cpp/libpqxx/orm/manager.h"

#include "libraries/utils/cpp/expected_type.hpp"

#include "data_dispatcher.h"
#include "errors.h"

namespace utils::qt::models {

    namespace _detail {

        class GenericTableModel : public QAbstractTableModel {
            Q_OBJECT

          public:
            explicit GenericTableModel(utils::libpqxx::ConnectionPool& pool, QObject* parent = nullptr);

          public slots:
            void refresh_all();
            void refresh_row(int row);
            void refresh_item(utils::db::Id id);

          protected:
            virtual void _refresh_all() = 0;

            /// Updates data in row `row`
            virtual void _refresh_row(int row) = 0;
            virtual void _refresh_item(utils::db::Id id) = 0;

          protected:
            utils::libpqxx::ConnectionPool& pool;
        };

    } // namespace _detail

    template <class TModel, typename TColumn> class TableModel : public _detail::GenericTableModel {
      protected:
        using ModelData = utils::db::ModelData<TModel>;
        using ModelManager = utils::db::ModelManager<TModel>;
        static constexpr std::string_view name = utils::type_name<TableModel<TModel, TColumn>>();

      public:
        explicit TableModel(utils::libpqxx::ConnectionPool& pool, QObject* parent = nullptr)
            : _detail::GenericTableModel{pool, parent} {};

        int rowCount(const QModelIndex& parent = QModelIndex()) const override { return items.size(); }

        int columnCount(const QModelIndex& parent = QModelIndex()) const override {
            return magic_enum::enum_count<TColumn>();
        }

        QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override {
            QVariant result = QVariant();

            if (role == Qt::DisplayRole && orientation == Qt::Horizontal) { // H
                TColumn column = magic_enum::enum_value<TColumn>(section);
                result = QString::fromStdString(std::string(magic_enum::enum_name(column)));
            } else if (role == Qt::DisplayRole && orientation == Qt::Vertical) { // V
                return QString::fromStdString(std::format("{}", items[section].id));
            } else {
                // other stuff
            }
            return result;
        }

        QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override {
            QVariant result = QVariant();

            int row = index.row();
            int column_idx = index.column();

            if (!index.isValid() || row >= rowCount() || column_idx >= columnCount()) {
                return result;
            }

            TColumn column = magic_enum::enum_value<TColumn>(column_idx);
            const auto& item = items.at(row);

            switch (role) {
            case Qt::DisplayRole:
                return DataDispatcher<TModel, TColumn, Qt::DisplayRole>::data(item, column);
            case Qt::FontRole:
                return DataDispatcher<TModel, TColumn, Qt::FontRole>::data(item, column);
            case Qt::TextAlignmentRole:
                return DataDispatcher<TModel, TColumn, Qt::TextAlignmentRole>::data(item, column);
            case Qt::BackgroundRole:
                return DataDispatcher<TModel, TColumn, Qt::BackgroundRole>::data(item, column);
                // default:
                //     break;
            }

            return result;
        }

        const TModel& get(int row) const {
            SPDLOG_DEBUG("{}::get(row={})", name, row);
            return items.at(row);
        };

        utils::ExpectedType<std::reference_wrapper<const TModel>, ErrorItemNotFound>
        get(const ModelData::Id& id) const {
            SPDLOG_DEBUG("{}::get(id={})", name, id);

            auto found = std::find_if(items.begin(), items.end(), [&id](const auto& item) { return item.id == id; });
            if (found == items.end()) {
                SPDLOG_ERROR(" - Unexpected: {} item with id {} not found in table {}!", ModelData::name, id, name);
                return tl::unexpected{ErrorItemNotFound{}};
            }

            return {*found};
        };

      protected:
        void _refresh_all() override {
            SPDLOG_DEBUG("{}::_refresh_all", name);

            SPDLOG_TRACE(" - fetch all the items for this model");
            ModelManager manager{pool};
            auto all_items = manager.all();
            if (!all_items) {
                SPDLOG_ERROR("Error refreshing items: {}", all_items.error());
                // TODO: Communicate error to user
                return;
            }
            this->items = std::move(all_items.value());
        };

        void _refresh_row(int row) override final {
            SPDLOG_DEBUG("{}::_refresh_one(row={})", name, row);
            const typename ModelData::Id& id = items.at(row).id;
            this->_refresh_one(id, row);
            // Emitting the data changed is responsibility of the leaf implementation, only them
            // know which columns have been modified and for which Qt::ItemDataRole.
            // By default, here I could notify the full row has been updated
        };

        void _refresh_item(utils::db::Id id) override final {
            SPDLOG_DEBUG("{}::_refresh_one(id={})", name, id);
            auto found = std::find_if(items.begin(), items.end(), [&id](const auto& item) { return item.id == id; });
            if (found == items.end()) {
                SPDLOG_WARN("Item with id='{}' not found (total {} items)", id, items.size());
                // TODO: Communicate error to user
                return;
            }

            int row = std::distance(items.begin(), found);
            this->_refresh_one(id, row);
            // Emitting the data changed is responsibility of the leaf implementation, only them
            // know which columns have been modified and for which Qt::ItemDataRole.
            // By default, here I could notify the full row has been updated
        }

        virtual void _refresh_one(const ModelData::Id& id, int row) {
            SPDLOG_WARN("{}::_refresh_one(id={}, row={}) -- empty implementation", name, id, row);

            ModelManager manager{pool};
            auto new_item = manager.get(id);
            if (!new_item) {
                SPDLOG_ERROR("Error refreshing item: {}", new_item.error());
                // TODO: Communicate error to user
                return;
            }
            this->items.at(row) = std::move(new_item.value());

            // Emit a signal to notify that the row has been modified
            QModelIndex topLeft = this->createIndex(row, 0);
            QModelIndex bottomRight = this->createIndex(row, this->columnCount());
            emit dataChanged(topLeft, bottomRight);
        };

      protected:
        std::vector<TModel> items;
    };

    template <class TParent, class TModel, typename TColumn>
    class FilteredTableModel : public TableModel<TModel, TColumn> {
        using ModelManager = TableModel<TModel, TColumn>::ModelManager;
        static constexpr std::string_view name = utils::type_name<FilteredTableModel<TParent, TModel, TColumn>>();

      public:
        explicit FilteredTableModel(const TParent& parent_, utils::libpqxx::ConnectionPool& pool,
                                    QObject* parent_object = nullptr)
            : TableModel<TModel, TColumn>{pool, parent_object}, parent{parent_} {};

        const TParent& get_parent() const { return parent; };

        QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override {
            QVariant result = QVariant();

            int row = index.row();
            int column_idx = index.column();

            if (!index.isValid() || row >= this->rowCount() || column_idx >= this->columnCount()) {
                return result;
            }

            TColumn column = magic_enum::enum_value<TColumn>(column_idx);
            const auto& item = this->items.at(row);

            switch (role) {
            case Qt::DisplayRole:
                return DataDispatcher<TModel, TColumn, Qt::DisplayRole>::data(item, column, parent);
            case Qt::FontRole:
                return DataDispatcher<TModel, TColumn, Qt::FontRole>::data(item, column, parent);
            case Qt::TextAlignmentRole:
                return DataDispatcher<TModel, TColumn, Qt::TextAlignmentRole>::data(item, column, parent);
            case Qt::BackgroundRole:
                return DataDispatcher<TModel, TColumn, Qt::BackgroundRole>::data(item, column, parent);
                // default:
                //     break;
            }

            return result;
        }

      protected:
        void _refresh_all() override {
            SPDLOG_DEBUG("{}::_refresh_all", name);

            SPDLOG_TRACE(" - fetch all the items for this model");
            ModelManager manager{this->pool};
            auto all_items = manager.filter_by_fk(parent);
            if (!all_items) {
                SPDLOG_ERROR("Error refreshing items: {}", all_items.error());
                // TODO: Communicate error to user
                return;
            }
            this->items = std::move(all_items.value());
        };

      protected:
        const TParent& parent;
    };
} // namespace utils::qt::models

#pragma once

#include <vector>

#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/expected_type/expected_type.hpp"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "errors.h"
#include "model.h"

namespace utils::db {

    template <typename TModel> class ModelManager {
        using TModelData = ModelData<TModel>;

      public:
        ModelManager(utils::libpqxx::ConnectionPool& pool) : pool{pool} {}

        /// Returns all the rows from the database
        ExpectedType<std::vector<TModel>, DatabaseError> all() {
            return pool.with_conn<ExpectedType<std::vector<TModel>, DatabaseError>>(
                [](pqxx::connection& conn) -> ExpectedType<std::vector<TModel>, DatabaseError> {
                    try {
                        SPDLOG_DEBUG("Get all {}", TModelData::name);

                        pqxx::work tx(conn);
                        std::vector<TModel> ret = ModelManager::_all(tx);
                        SPDLOG_TRACE("Found {} {}", ret.size(), TModelData::name);
                        return ExpectedType<std::vector<TModel>, DatabaseError>{std::move(ret)};
                    } catch (const std::exception& e) {
                        return tl::unexpected(DatabaseError{
                            std::format("Failed to fetch all '{}' items: {}", TModelData::name, e.what())});
                    }
                });
        }

        /// Returns the row that matches the given `id`.
        ExpectedType<TModel, DatabaseError, ErrorNotFound, ErrorMultipleFound> get(const typename TModelData::Id& id) {
            return pool.with_conn<ExpectedType<TModel, DatabaseError, ErrorNotFound, ErrorMultipleFound>>(
                [&id](
                    pqxx::connection& conn) -> ExpectedType<TModel, DatabaseError, ErrorNotFound, ErrorMultipleFound> {
                    try {
                        SPDLOG_DEBUG("Get {} with id {}", TModelData::name, id);

                        pqxx::work tx(conn);
                        return ModelManager::_get(tx, id);
                    } catch (const std::exception& e) {
                        return tl::unexpected(
                            DatabaseError{std::format("Failed to fetch '{}' item: {}", TModelData::name, e.what())});
                    }
                });
        }

        /// Returns all the rows that has a foreign key to the given `TParentModel` `id`
        template <typename TParentModel>
        ExpectedType<std::vector<TModel>, DatabaseError> filter_by_fk(const typename ModelData<TParentModel>::Id& id) {
            return pool.with_conn<ExpectedType<std::vector<TModel>, DatabaseError>>(
                [&id](pqxx::connection& conn) -> ExpectedType<std::vector<TModel>, DatabaseError> {
                    try {
                        SPDLOG_DEBUG("Get all the {} that are related to the {} with id {}", TModelData::name,
                                     ModelData<TParentModel>::name, id);

                        pqxx::work tx(conn);
                        std::vector<TModel> all_items = ModelManager::_filter_by_fk<TParentModel>(tx, id);
                        return {std::move(all_items)};
                    } catch (const std::exception& e) {
                        return tl::unexpected(
                            DatabaseError{std::format("Failed to fetch '{}' items: {}", TModelData::name, e.what())});
                    }
                });
        }

        /// Returns all the rows that has a foreign key to the give `TParentModel`
        template <typename TParentModel>
        ExpectedType<std::vector<TModel>, DatabaseError> filter_by_fk(const TParentModel& parent) {
            return this->filter_by_fk<TParentModel>(parent.id);
        }

        /// Creates a new TModel, and returns its id
        ExpectedType<Id, DatabaseError, ErrorInvalidInput> create(TModel&& new_instance) {
            if (!is_null(new_instance.id)) {
                return tl::unexpected{ErrorInvalidInput{"Cannot create an instance if the 'id' is already given"}};
            }

            return pool.with_conn<ExpectedType<Id, DatabaseError, ErrorInvalidInput>>(
                [&new_instance](pqxx::connection& conn) -> ExpectedType<Id, DatabaseError, ErrorInvalidInput> {
                    try {
                        SPDLOG_DEBUG("Create new instance of model {}", TModelData::name);

                        pqxx::work tx(conn);
                        utils::db::Id new_id = ModelManager::_create(tx, std::move(new_instance));
                        tx.commit();
                        return {std::move(new_id)};
                    } catch (const std::exception& e) {
                        return tl::unexpected(DatabaseError{std::format(
                            "Failed to create new instance of model '{}': {}", TModelData::name, e.what())});
                    }
                });
        };

        static Id _create(pqxx::work&, TModel&&);
        static Id _create(pqxx::work&, const TModel&);

      protected:
        static std::vector<TModel> _all(pqxx::work&);

        static ExpectedType<TModel, ErrorNotFound, ErrorMultipleFound> _get(pqxx::work&,
                                                                            const typename TModelData::Id&);

        template <typename TParentModel>
        static std::vector<TModel> _filter_by_fk(pqxx::work&, const typename ModelData<TParentModel>::Id&);

      protected:
        utils::libpqxx::ConnectionPool& pool;
    };

} // namespace utils::db

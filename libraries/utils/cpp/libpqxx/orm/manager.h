#pragma once

#include <vector>

#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/expected_type.hpp"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "errors.h"
#include "model.h"

namespace utils::db {

    template <typename TModel> class ModelManager {
        using TModelData = Model<TModel>;

      public:
        ModelManager(utils::libpqxx::ConnectionPool& pool) : pool{pool} {}

        ExpectedType<std::vector<TModel>, DatabaseError> all() {
            return pool.with_conn<ExpectedType<std::vector<TModel>, DatabaseError>>(
                [](pqxx::connection& conn) -> ExpectedType<std::vector<TModel>, DatabaseError> {
                    try {
                        SPDLOG_DEBUG("Get all {}", TModelData::name);

                        pqxx::work tx(conn);
                        std::vector<TModel> ret = TModelData::get_all(tx);
                        SPDLOG_TRACE("Found {} {}", ret.size(), TModelData::name);
                        return {ret};
                    } catch (const std::exception& e) {
                        SPDLOG_ERROR("Failed to fetch all the {}: {}", TModelData::name, e.what());
                        return tl::unexpected(DatabaseError{});
                    }
                });
        }

        ExpectedType<TModel, DatabaseError, ErrorNotFound, ErrorMultipleFound> get(const typename TModelData::Id& id) {
            return pool.with_conn<ExpectedType<TModel, DatabaseError, ErrorNotFound, ErrorMultipleFound>>(
                [&id](
                    pqxx::connection& conn) -> ExpectedType<TModel, DatabaseError, ErrorNotFound, ErrorMultipleFound> {
                    try {
                        SPDLOG_DEBUG("Get {} with id {}", TModelData::name, id);

                        pqxx::work tx(conn);
                        return TModelData::get(tx, id);
                        // return tl::unexpected(DatabaseError{});
                    } catch (const std::exception& e) {
                        SPDLOG_ERROR("Failed to fetch {} model: {}", TModelData::name, e.what());
                        return tl::unexpected(DatabaseError{});
                    }
                });
        }

      protected:
        utils::libpqxx::ConnectionPool& pool;
    };

} // namespace utils::db

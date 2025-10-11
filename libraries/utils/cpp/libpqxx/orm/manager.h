#pragma once

#include "libraries/utils/cpp/expected_type.hpp"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"
#include <spdlog/spdlog.h>
#include <vector>

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
                        pqxx::work tx(conn);
                        SPDLOG_DEBUG("Get all {}", TModelData::name);

                        std::vector<TModel> ret = TModelData::get_all(tx);
                        SPDLOG_TRACE("Found {} {}", ret.size(), TModelData::name);
                        return {ret};
                    } catch (const std::exception& e) {
                        SPDLOG_ERROR("Failed to fetch all the {}: {}", TModelData::name, e.what());
                        return tl::unexpected(DatabaseError{});
                    }
                });
        }

        ExpectedType<TModel, DatabaseError> get(typename TModelData::Id id);

      protected:
        utils::libpqxx::ConnectionPool& pool;
    };

} // namespace utils::db

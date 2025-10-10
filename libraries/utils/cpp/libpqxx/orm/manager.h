#pragma once

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

        Expected<std::vector<TModel>, DatabaseError> all() {
            return pool.with_conn<Expected<std::vector<TModel>, DatabaseError>>(
                [](pqxx::connection& conn) -> Expected<std::vector<TModel>, DatabaseError> {
                    try {
                        pqxx::work tx(conn);
                        SPDLOG_DEBUG("Get all accounts");

                        std::vector<TModel> ret;
                        auto query = std::format("SELECT *"
                                                 " FROM {};", TModelData::table_name);
                        SPDLOG_TRACE(query);
                        for (const auto& row: tx.exec(query)) {
                          auto instance = TModelData::parse(row);
                        }
                        // for (auto [id, name, description, identifier, ccy, open, close, type_id, type_name,
                        //            custodian_id, custodian_name, is_numerable] :
                        //      tx.query<Id, std::string, std::optional<std::string>, std::optional<std::string>,
                        //               std::string, utils::libpqxx::Date, std::optional<utils::libpqxx::Date>, Id,
                        //               std::string, Id, std::string, bool>(query)) {
                        //     ret.emplace_back(Account{.id = id,
                        //                              .name = name,
                        //                              .description = description,
                        //                              .identifier = identifier,
                        //                              .ccy = Ccy{std::move(ccy)},
                        //                              .open = open,
                        //                              .close = close,
                        //                              .type = std::make_pair(type_id, type_name),
                        //                              .custodian = std::make_pair(custodian_id, custodian_name),
                        //                              .is_numerable = is_numerable});
                        // }
                        SPDLOG_TRACE("Found {} accounts", ret.size());
                        return {ret};
                    } catch (const std::exception& e) {
                        SPDLOG_ERROR("Failed to fetch all the accounts: {}", e.what());
                        return tl::unexpected(DatabaseError{});
                    }
                });
        }

        Expected<TModel, DatabaseError, ErrorNotFound, ErrorMultipleFound> get(typename TModelData::Id id);

      protected:
        utils::libpqxx::ConnectionPool& pool;
    };

} // namespace utils::db

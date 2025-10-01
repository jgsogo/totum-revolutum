#include "hierarchy_tree.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

namespace {
    template <typename T> std::vector<T> get_all(pqxx::connection& conn, std::string_view table) {
        pqxx::work tx(conn);
        SPDLOG_DEBUG("Get all hierarchy tree elements from table '{}'", table);

        std::vector<T> ret;
        auto query = std::format("SELECT id, name, description, is_abstract, unique_name"
                                 " FROM {};",
                                 table);
        SPDLOG_TRACE(query);
        for (auto [id, name, description, is_abstract, unique_name] :
             tx.query<Id, std::string, std::optional<std::string>, bool, std::optional<std::string>>(query)) {
            ret.emplace_back(T{
                .id = id,
                .name = name,
                .description = description,
                .is_abstract = is_abstract,
                .unique_name = unique_name,
            });
        }
        SPDLOG_TRACE("Found {} items", ret.size());
        return {ret};
    }

    std::vector<std::string> get_breadcrumb(pqxx::connection& conn, Id id, std::string_view table) {
        pqxx::work tx(conn);
        auto query = std::format(""
                                 "SELECT i.id AS source_id, "
                                 "       i.name AS source_name, "
                                 "       r.id AS referenced_id, "
                                 "       r.name AS referenced_name "
                                 "FROM {} i "
                                 "JOIN LATERAL unnest(string_to_array(i.tn_ancestors_pks, ',')) AS ref_id ON TRUE "
                                 "JOIN {} r ON r.id = ref_id::INT "
                                 "WHERE i.id = $1;",
                                 table, table);
        SPDLOG_TRACE(query);
        std::vector<std::string> ret;
        for (auto [id, source, ref_id, target] : tx.query<Id, std::string, Id, std::string>(query, pqxx::params{id})) {
            ret.emplace_back(std::move(target));
        }
        return ret;
    }
} // namespace

template <> tl::expected<std::vector<AccountType>, Error> HierarchyTreeManager<AccountType>::all() {
    return pool.with_conn<tl::expected<std::vector<AccountType>, Error>>(
        [](pqxx::connection& conn) -> tl::expected<std::vector<AccountType>, Error> {
            try {
                return {get_all<AccountType>(conn, ACCOUNT_TYPE_TABLE)};
            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch all AccountType: {}", e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

template <> tl::expected<std::vector<std::string>, Error> HierarchyTreeManager<AccountType>::breadcrumb(Id id) {
    return pool.with_conn<tl::expected<std::vector<std::string>, Error>>(
        [id](pqxx::connection& conn) -> tl::expected<std::vector<std::string>, Error> {
            try {
                return {get_breadcrumb(conn, id, ACCOUNT_TYPE_TABLE)};
            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch breadcrumb for account type {}: {}", id, e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

template <> tl::expected<std::vector<MovementType>, Error> HierarchyTreeManager<MovementType>::all() {
    return pool.with_conn<tl::expected<std::vector<MovementType>, Error>>(
        [](pqxx::connection& conn) -> tl::expected<std::vector<MovementType>, Error> {
            try {
                return {get_all<MovementType>(conn, MOVEMENTTYPE_TABLE)};
            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch all MovementType: {}", e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

template <> tl::expected<std::vector<std::string>, Error> HierarchyTreeManager<MovementType>::breadcrumb(Id id) {
    return pool.with_conn<tl::expected<std::vector<std::string>, Error>>(
        [id](pqxx::connection& conn) -> tl::expected<std::vector<std::string>, Error> {
            try {
                return {get_breadcrumb(conn, id, MOVEMENTTYPE_TABLE)};
            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch breadcrumb for movement type {}: {}", id, e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

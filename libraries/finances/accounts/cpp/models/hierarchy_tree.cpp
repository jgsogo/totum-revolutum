#include "hierarchy_tree.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

namespace utils::db {

    namespace _impl {
        template <typename T> std::vector<T> get_all(pqxx::work& tx, std::string_view table) {
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

        template <typename T>
        ExpectedType<T, ErrorNotFound, ErrorMultipleFound> get(pqxx::work& tx, std::string_view table,
                                                               const utils::db::Id& item_id) {
            SPDLOG_DEBUG("Get one hierarchy tree element from table '{}' with id '{}'", table, item_id);

            auto query = std::format("SELECT id, name, description, is_abstract, unique_name"
                                     " FROM {}"
                                     " WHERE id = $1;",
                                     table);
            SPDLOG_TRACE(query);

            auto r = tx.exec(query, pqxx::params{item_id}).one_row();
            auto [id, name, description, is_abstract, unique_name] =
                r.as<Id, std::string, std::optional<std::string>, bool, std::optional<std::string>>();
            return {T{
                .id = id,
                .name = name,
                .description = description,
                .is_abstract = is_abstract,
                .unique_name = unique_name,
            }};
        }
    } // namespace _impl

    template <> std::vector<AccountType> ModelManager<AccountType>::_all(pqxx::work& tx) {
        return _impl::get_all<AccountType>(tx, ACCOUNT_TYPE_TABLE);
    }

    template <> std::vector<MovementType> ModelManager<MovementType>::_all(pqxx::work& tx) {
        return _impl::get_all<MovementType>(tx, MOVEMENTTYPE_TABLE);
    }

    template <>
    ExpectedType<MovementType, ErrorNotFound, ErrorMultipleFound>
    ModelManager<MovementType>::_get(pqxx::work& tx, const decltype(MovementType::id)& id) {
        return _impl::get<MovementType>(tx, MOVEMENTTYPE_TABLE, id);
    }

} // namespace utils::db

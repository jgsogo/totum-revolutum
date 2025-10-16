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
    } // namespace _impl

    template <> std::vector<AccountType> ModelManager<AccountType>::_all(pqxx::work& tx) {
        return _impl::get_all<AccountType>(tx, ACCOUNT_TYPE_TABLE);
    }

    template <> std::vector<MovementType> ModelManager<MovementType>::_all(pqxx::work& tx) {
        return _impl::get_all<MovementType>(tx, MOVEMENTTYPE_TABLE);
    }

} // namespace utils::db

#pragma once

#include <optional>
#include <string>

#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/libpqxx/orm/manager.h"
#include "libraries/utils/cpp/libpqxx/orm/model.h"
#include "libraries/utils/cpp/string_literal.hpp"

#include "model_manager.hpp"
#include "types/id.h"

namespace finances::accounts::models {

    template <typename Tag> struct HierarchyTree {
        Id id;
        std::string name;
        std::optional<std::string> description;
        bool is_abstract;
        std::optional<std::string> unique_name;
    };

    using AccountType = HierarchyTree<class AccountTypeTag>;
    using MovementType = HierarchyTree<class MovementTypeTag>;

} // namespace finances::accounts::models

namespace utils::db {

    namespace _impl {

        template <typename THierarchyTree, utils::StringLiteral TABLE_NAME>
        class HierarchyTreeManager : public ModelManager<THierarchyTree> {
          public:
            using ModelManager<THierarchyTree>::ModelManager;
            using Id = decltype(THierarchyTree::id);
            using ModelManager<THierarchyTree>::pool;

            /// Returns the breadcrumb for the given `THierarchyTree` model. The breadcrumb doesn't include the element
            /// itself.
            ExpectedType<std::vector<std::pair<Id, std::string>>, DatabaseError> breadcrumb(const Id& id) {
                return pool.template with_conn<ExpectedType<std::vector<std::pair<Id, std::string>>, DatabaseError>>(
                    [id](pqxx::connection& conn)
                        -> ExpectedType<std::vector<std::pair<Id, std::string>>, DatabaseError> {
                        try {
                            pqxx::work tx(conn);
                            auto query = std::format(
                                ""
                                "SELECT i.id AS source_id, "
                                "       i.name AS source_name, "
                                "       r.id AS referenced_id, "
                                "       r.name AS referenced_name "
                                "FROM {} i "
                                "JOIN LATERAL unnest(string_to_array(i.tn_ancestors_pks, ',')) AS ref_id ON TRUE "
                                "JOIN {} r ON r.id = ref_id::INT "
                                "WHERE i.id = $1;",
                                TABLE_NAME, TABLE_NAME);
                            SPDLOG_TRACE(query);

                            std::vector<std::pair<Id, std::string>> ret;
                            for (auto [source_id, source, target_id, target] :
                                 tx.query<Id, std::string, Id, std::string>(query, pqxx::params{id})) {
                                ret.emplace_back(std::make_pair(target_id, std::move(target)));
                            }
                            return ret;

                        } catch (const std::exception& e) {
                            SPDLOG_ERROR("Failed to fetch breadcrumb for model {} (pk={}): {}",
                                         utils::type_name<THierarchyTree>(), id, e.what());
                            return tl::unexpected(DatabaseError{});
                        }
                    });
            }
        };

    } // namespace _impl

    using AccountTypeManager = _impl::HierarchyTreeManager<finances::accounts::models::AccountType,
                                                           finances::accounts::models::ACCOUNT_TYPE_TABLE>;
    using MovementTypeManager = _impl::HierarchyTreeManager<finances::accounts::models::MovementType,
                                                            finances::accounts::models::MOVEMENTTYPE_TABLE>;

    template <>
    std::vector<finances::accounts::models::AccountType>
    ModelManager<finances::accounts::models::AccountType>::_all(pqxx::work&);

    template <>
    std::vector<finances::accounts::models::MovementType>
    ModelManager<finances::accounts::models::MovementType>::_all(pqxx::work&);

} // namespace utils::db

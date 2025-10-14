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

    template <typename Tag> class HierarchyTreeManager;

    template <typename Tag> struct HierarchyTree {
        using Manager = HierarchyTreeManager<HierarchyTree<Tag>>;

        Id id;
        std::string name;
        std::optional<std::string> description;
        bool is_abstract;
        std::optional<std::string> unique_name;
    };

    using AccountType = HierarchyTree<class AccountTypeTag>;
    using MovementType = HierarchyTree<class MovementTypeTag>;

    template <typename THierarchyTree> class HierarchyTreeManager : public ModelManager<THierarchyTree> {
      public:
        tl::expected<std::vector<THierarchyTree>, Error> all();
        tl::expected<std::vector<std::string>, Error> breadcrumb(Id id);
    };

    template <> tl::expected<std::vector<AccountType>, Error> HierarchyTreeManager<AccountType>::all();
    template <> tl::expected<std::vector<std::string>, Error> HierarchyTreeManager<AccountType>::breadcrumb(Id id);

    template <> tl::expected<std::vector<MovementType>, Error> HierarchyTreeManager<MovementType>::all();
    template <> tl::expected<std::vector<std::string>, Error> HierarchyTreeManager<MovementType>::breadcrumb(Id id);

} // namespace finances::accounts::models

namespace utils::db {

    // template <>
    // constexpr std::string_view ModelData<finances::accounts::models::AccountType>::table_name =
    //     "finances_accounts_accounttype";

    namespace _impl {

        template <typename THierarchyTree, utils::StringLiteral TABLE_NAME>
        class HierarchyTreeManager : public ModelManager<THierarchyTree> {
          public:
            using ModelManager<THierarchyTree>::ModelManager;
            using Id = decltype(THierarchyTree::id);
            using ModelManager<THierarchyTree>::pool;

            ExpectedType<std::vector<std::string>, DatabaseError> breadcrumb(const Id& id) {
                return pool.template with_conn<ExpectedType<std::vector<std::string>, DatabaseError>>(
                    [id](pqxx::connection& conn) -> ExpectedType<std::vector<std::string>, DatabaseError> {
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

                            std::vector<std::string> ret;
                            for (auto [id, source, ref_id, target] :
                                 tx.query<Id, std::string, Id, std::string>(query, pqxx::params{id})) {
                                ret.emplace_back(std::move(target));
                            }
                            return ret;

                        } catch (const std::exception& e) {
                            SPDLOG_ERROR("Failed to fetch breadcrumb for account type {}: {}", id, e.what());
                            return tl::unexpected(DatabaseError{});
                        }
                    });
            }
        };
    } // namespace _impl

    using AccountTypeManager =
        _impl::HierarchyTreeManager<finances::accounts::models::AccountType, "finances_accounts_accounttype">;
    using MovementTypeManager =
        _impl::HierarchyTreeManager<finances::accounts::models::MovementType, "finances_accounts_movementtype">;

} // namespace utils::db

#include "hierarchy_tree.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

// impl AccountType {
//     pub fn get_breadcrumbs(&self, conn: &mut PgConnection) -> Result<Vec<String>, diesel::result::Error> {
//         let me_pk = vec![self.id];
//         let all_pks = [self.tn_ancestors_pks.nodes.clone(), me_pk].concat();
//         let breadcrumbs = Self::all()
//             .filter(accounttype_by_pks(&all_pks))
//             .select((
//                 crate::schema::finances_accounts_accounttype::id,
//                 crate::schema::finances_accounts_accounttype::name,
//             ))
//             .load::<(i64, String)>(conn)?;

//         Ok(reorder_breadcrumbs(breadcrumbs, &all_pks))
//     }
// }

namespace {
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

// tl::expected<std::vector<Movement>, Error> MovementManager::all(Id account_id) {
//     return pool.with_conn<tl::expected<std::vector<Movement>, Error>>(
//         [account_id](pqxx::connection& conn) -> tl::expected<std::vector<Movement>, Error> {
//             try {
//                 pqxx::work tx(conn);
//                 SPDLOG_DEBUG("Get all movements for account_id {}", account_id);

//                 auto query =
//                     std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name"
//                                 " FROM {} AS m"
//                                 "   LEFT JOIN {} AS t ON m.type_id = t.id"
//                                 "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
//                                 " WHERE m.account_id = $1"
//                                 " ORDER BY m.date_value DESC",
//                                 MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE);
//                 SPDLOG_TRACE(query);
//                 std::vector<Movement> ret;
//                 for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name] :
//                      tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id, std::string>(
//                          query, pqxx::params{account_id})) {
//                     ret.emplace_back(Movement{.id = id,
//                                               .transaction = std::make_pair(transaction_id, transaction_name),
//                                               .type = std::make_pair(type_id, type_name),
//                                               .direction = direction,
//                                               .account_id = account_id,
//                                               .date_value = date_value,
//                                               .amount = amount});
//                 }
//                 SPDLOG_TRACE("Found {} movements for account {}", ret.size(), account_id);
//                 return {ret};
//             } catch (const std::exception& e) {
//                 SPDLOG_ERROR("Failed to fetch all movements for account {}: {}", account_id, e.what());
//                 return tl::unexpected(Error::DBError);
//             }
//         });
// }

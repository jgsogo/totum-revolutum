#include "movement.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

namespace utils::db {

    template <>
    template <>
    std::vector<Movement>
    utils::db::ModelManager<Movement>::_filter_by_fk<Account>(pqxx::work& tx,
                                                              const ModelData<Account>::Id& account_id) {
        SPDLOG_DEBUG("Get all movements for account_id {}", account_id);

        auto query = std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name"
                                 " FROM {} AS m"
                                 "   LEFT JOIN {} AS t ON m.type_id = t.id"
                                 "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
                                 " WHERE m.account_id = $1"
                                 " ORDER BY m.date_value DESC",
                                 MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE);
        SPDLOG_TRACE(query);
        std::vector<Movement> ret;
        for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name] :
             tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id, std::string>(
                 query, pqxx::params{account_id})) {
            ret.emplace_back(Movement{.id = id,
                                      .transaction = std::make_pair(transaction_id, transaction_name),
                                      .type = std::make_pair(type_id, type_name),
                                      .direction = direction,
                                      .account_id = account_id,
                                      .date_value = date_value,
                                      .amount = amount});
        }
        SPDLOG_TRACE("Found {} movements for account {}", ret.size(), account_id);
        return ret;
    }

} // namespace utils::db

tl::expected<std::vector<Movement>, Error> MovementManager::all(Id account_id) {
    return pool.with_conn<tl::expected<std::vector<Movement>, Error>>(
        [account_id](pqxx::connection& conn) -> tl::expected<std::vector<Movement>, Error> {
            try {
                pqxx::work tx(conn);
                SPDLOG_DEBUG("Get all movements for account_id {}", account_id);

                auto query =
                    std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name"
                                " FROM {} AS m"
                                "   LEFT JOIN {} AS t ON m.type_id = t.id"
                                "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
                                " WHERE m.account_id = $1"
                                " ORDER BY m.date_value DESC",
                                MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE);
                SPDLOG_TRACE(query);
                std::vector<Movement> ret;
                for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name] :
                     tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id, std::string>(
                         query, pqxx::params{account_id})) {
                    ret.emplace_back(Movement{.id = id,
                                              .transaction = std::make_pair(transaction_id, transaction_name),
                                              .type = std::make_pair(type_id, type_name),
                                              .direction = direction,
                                              .account_id = account_id,
                                              .date_value = date_value,
                                              .amount = amount});
                }
                SPDLOG_TRACE("Found {} movements for account {}", ret.size(), account_id);
                return {ret};
            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch all movements for account {}: {}", account_id, e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

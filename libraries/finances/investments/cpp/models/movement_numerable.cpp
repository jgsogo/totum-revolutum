#include "movement_numerable.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

#include "constants.h"

using namespace finances::investments::models;
using namespace finances::accounts::models;

tl::expected<std::vector<MovementNumerable>, finances::accounts::models::Error>
MovementNumerableManager::all(finances::accounts::models::Id account_id) {
    return pool.with_conn<tl::expected<std::vector<MovementNumerable>, Error>>(
        [account_id](pqxx::connection& conn) -> tl::expected<std::vector<MovementNumerable>, Error> {
            try {
                pqxx::work tx(conn);
                SPDLOG_DEBUG("Get all movements for account_id {}", account_id);

                auto query =
                    std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name, "
                                "mn.movement_ptr_id, mn.quantity, mn.unit_value"
                                " FROM {} AS mn"
                                "   LEFT JOIN {} AS m ON mn.movement_ptr_id = m.id"
                                "   LEFT JOIN {} AS t ON m.type_id = t.id"
                                "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
                                " WHERE m.account_id = $1"
                                " ORDER BY m.date_value DESC",
                                MOVEMENT_NUMERABLE_TABLE, MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE);
                SPDLOG_TRACE(query);
                std::vector<MovementNumerable> ret;
                for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name,
                           mn_id, quantity, unit_value] :
                     tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id, std::string, Id,
                              Amount, Amount>(query, pqxx::params{account_id})) {
                    ret.emplace_back(MovementNumerable{
                        .movement = Movement{.id = id,
                                             .transaction = std::make_pair(transaction_id, transaction_name),
                                             .type = std::make_pair(type_id, type_name),
                                             .direction = direction,
                                             .account_id = account_id,
                                             .date_value = date_value,
                                             .amount = amount},
                        .id = mn_id,
                        .quantity = quantity,
                        .unit_value = unit_value,
                    });
                }
                SPDLOG_TRACE("Found {} movements for account {}", ret.size(), account_id);
                return {ret};
            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch all movements for account {}: {}", account_id, e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

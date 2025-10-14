#include "movement_dividend.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

#include "constants.h"

using namespace finances::investments::models;
using namespace finances::accounts::models;

// tl::expected<std::vector<MovementDividend>, finances::accounts::models::Error>
// MovementDividendManager::all(finances::accounts::models::Id account_id) {
//     return pool.with_conn<tl::expected<std::vector<MovementDividend>, Error>>(
//         [account_id](pqxx::connection& conn) -> tl::expected<std::vector<MovementDividend>, Error> {
//             try {
//                 pqxx::work tx(conn);
//                 SPDLOG_DEBUG("Get all movements for account_id {}", account_id);

//                 auto query =
//                     std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name, "
//                                 "mn.movement_ptr_id, mn.ex_dividend_date, mn.unit_value, s.date_value, s.quantity"
//                                 " FROM {} AS mn"
//                                 "   LEFT JOIN {} AS m ON mn.movement_ptr_id = m.id"
//                                 "   LEFT JOIN {} AS t ON m.type_id = t.id"
//                                 "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
//                                 "    LEFT JOIN LATERAL ("
//                                 "       SELECT s.date_value, sn.quantity"
//                                 "       FROM {} AS s"
//                                 "           LEFT JOIN {} AS sn ON sn.snapshot_ptr_id = s.id"
//                                 "       WHERE s.account_id = $1 AND s.date_value <= m.date_value"
//                                 "       ORDER BY s.date_value DESC"
//                                 "       LIMIT 1"
//                                 "   ) AS s ON TRUE"
//                                 " WHERE m.account_id = $1"
//                                 " ORDER BY m.date_value DESC",
//                                 MOVEMENT_DIVIDEND_TABLE, MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE,
//                                 SNAPSHOT_TABLE, SNAPSHOT_NUMERABLE_TABLE);
//                 SPDLOG_TRACE(query);
//                 std::vector<MovementDividend> ret;
//                 for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name,
//                            mn_id, ex_dividend_date, unit_value, snapshot_date_value, snapshot_quantity] :
//                      tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id, std::string,
//                      Id,
//                               utils::libpqxx::Date, Amount, std::optional<utils::libpqxx::Date>,
//                               std::optional<Amount>>(
//                          query, pqxx::params{account_id})) {

//                     std::optional<std::pair<utils::libpqxx::Date, finances::accounts::models::Amount>> snapshot_data
//                     =
//                         std::nullopt;
//                     if (snapshot_date_value && snapshot_quantity) {
//                         snapshot_data = std::make_pair(std::move(snapshot_date_value.value()),
//                                                        std::move(snapshot_quantity.value()));
//                     } else {
//                         SPDLOG_WARN("Snapshot data missing for dividend! This is a database error!");
//                     }

//                     ret.emplace_back(MovementDividend{
//                         .movement = Movement{.id = id,
//                                              .transaction = std::make_pair(transaction_id, transaction_name),
//                                              .type = std::make_pair(type_id, type_name),
//                                              .direction = direction,
//                                              .account_id = account_id,
//                                              .date_value = date_value,
//                                              .amount = amount},
//                         .id = mn_id,
//                         .ex_dividend_date = ex_dividend_date,
//                         .unit_value = unit_value,
//                         .snapshot_data = snapshot_data,
//                     });
//                 }
//                 SPDLOG_TRACE("Found {} movements for account {}", ret.size(), account_id);
//                 return {ret};
//             } catch (const std::exception& e) {
//                 SPDLOG_ERROR("Failed to fetch all movements for account {}: {}", account_id, e.what());
//                 return tl::unexpected(Error::DBError);
//             }
//         });
// }

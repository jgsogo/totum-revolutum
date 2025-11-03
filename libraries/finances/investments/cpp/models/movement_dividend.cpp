#include "movement_dividend.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

#include "constants.h"

using namespace finances::investments::models;
using namespace finances::accounts::models;

namespace utils::db {

    namespace {
        std::vector<MovementDividend> filter_by_fk_id(pqxx::work& tx, const std::string& fk_key,
                                                      const utils::db::Id& id) {
            auto query = std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name, "
                                     "mn.movement_ptr_id, mn.ex_dividend_date, mn.unit_value, s.date_value, "
                                     "s.quantity, acc.id, acc.name, acc.ccy"
                                     " FROM {} AS mn"
                                     "   LEFT JOIN {} AS m ON mn.movement_ptr_id = m.id"
                                     "   LEFT JOIN {} AS t ON m.type_id = t.id"
                                     "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
                                     "   LEFT JOIN {} AS acc ON m.account_id = acc.id"
                                     "    LEFT JOIN LATERAL ("
                                     "       SELECT s.date_value, sn.quantity"
                                     "       FROM {} AS s"
                                     "           LEFT JOIN {} AS sn ON sn.snapshot_ptr_id = s.id"
                                     "       WHERE s.account_id = $1 AND s.date_value <= m.date_value"
                                     "       ORDER BY s.date_value DESC"
                                     "       LIMIT 1"
                                     "   ) AS s ON TRUE"
                                     " WHERE m.{} = $1"
                                     " ORDER BY m.date_value DESC",
                                     MOVEMENT_DIVIDEND_TABLE, MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE,
                                     ACCOUNT_TABLE, SNAPSHOT_TABLE, SNAPSHOT_NUMERABLE_TABLE, fk_key);
            SPDLOG_TRACE(query);

            std::vector<MovementDividend> ret;
            for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name, mn_id,
                       ex_dividend_date, unit_value, snapshot_date_value, snapshot_quantity, acc_id, acc_name,
                       acc_ccy] :
                 tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id, std::string, Id,
                          utils::libpqxx::Date, Amount, std::optional<utils::libpqxx::Date>, std::optional<Amount>, Id,
                          std::string, std::string>(query, pqxx::params{id})) {

                std::optional<std::pair<utils::libpqxx::Date, finances::accounts::models::Amount>> snapshot_data =
                    std::nullopt;
                if (snapshot_date_value && snapshot_quantity) {
                    snapshot_data =
                        std::make_pair(std::move(snapshot_date_value.value()), std::move(snapshot_quantity.value()));
                } else {
                    SPDLOG_ERROR("Snapshot data missing for dividend! This is a database error!");
                }

                ret.emplace_back(MovementDividend{
                    .movement = Movement{.id = id,
                                         .transaction = std::make_pair(transaction_id, transaction_name),
                                         .type = std::make_pair(type_id, type_name),
                                         .direction = direction,
                                         .account = std::make_pair(acc_id, acc_name),
                                         .date_value = date_value,
                                         .amount = Money{amount, Ccy{acc_ccy}}},
                    .id = mn_id,
                    .ex_dividend_date = ex_dividend_date,
                    .unit_value = Money{unit_value, Ccy{acc_ccy}},
                    .snapshot_data = snapshot_data,
                });
            }
            SPDLOG_TRACE("Found {} dividend movements", ret.size());
            return ret;
        }
    } // namespace

    template <>
    template <>
    std::vector<MovementDividend>
    utils::db::ModelManager<MovementDividend>::_filter_by_fk<Account>(pqxx::work& tx,
                                                                      const ModelData<Account>::Id& account_id) {
        SPDLOG_DEBUG("Get all dividend movements for account_id {}", account_id);
        return filter_by_fk_id(tx, "account_id", account_id);
    }

    template <>
    template <>
    std::vector<MovementDividend> utils::db::ModelManager<MovementDividend>::_filter_by_fk<Transaction>(
        pqxx::work& tx, const ModelData<Transaction>::Id& transaction_id) {
        SPDLOG_DEBUG("Get all dividend movements for transaction_id {}", transaction_id);
        return filter_by_fk_id(tx, "transaction_id", transaction_id);
    }

} // namespace utils::db

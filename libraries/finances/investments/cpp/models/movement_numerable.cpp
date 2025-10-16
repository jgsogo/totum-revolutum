#include "movement_numerable.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

#include "constants.h"

using namespace finances::investments::models;
using namespace finances::accounts::models;

namespace utils::db {

    template <>
    template <>
    std::vector<finances::investments::models::MovementNumerable>
    utils::db::ModelManager<finances::investments::models::MovementNumerable>::_filter_by_fk<
        finances::accounts::models::Account>(pqxx::work& tx,
                                             const ModelData<finances::accounts::models::Account>::Id& account_id) {
        SPDLOG_DEBUG("Get all movements for account_id {}", account_id);

        auto query =
            std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name, "
                        "mn.movement_ptr_id, mn.quantity, mn.unit_value"
                        " FROM {} AS mn"
                        "   LEFT JOIN {} AS m ON mn.movement_ptr_id = m.id"
                        "   LEFT JOIN {} AS t ON m.type_id = t.id"
                        "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
                        "   LEFT JOIN {} AS acc ON m.account_id = acc.id"
                        " WHERE m.account_id = $1"
                        " ORDER BY m.date_value DESC",
                        MOVEMENT_NUMERABLE_TABLE, MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE, ACCOUNT_TABLE);
        SPDLOG_TRACE(query);
        std::vector<MovementNumerable> ret;
        for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name, mn_id,
                   quantity, unit_value, acc_id, acc_name] :
             tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id, std::string, Id, Amount,
                      Amount, Id, std::string>(query, pqxx::params{account_id})) {
            ret.emplace_back(MovementNumerable{
                .movement = Movement{.id = id,
                                     .transaction = std::make_pair(transaction_id, transaction_name),
                                     .type = std::make_pair(type_id, type_name),
                                     .direction = direction,
                                     .account = std::make_pair(acc_id, acc_name),
                                     .date_value = date_value,
                                     .amount = amount},
                .id = mn_id,
                .quantity = quantity,
                .unit_value = unit_value,
            });
        }
        SPDLOG_TRACE("Found {} movements for account {}", ret.size(), account_id);
        return {ret};
    }

} // namespace utils::db

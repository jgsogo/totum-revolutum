#include "movement_numerable.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

#include "constants.h"

using namespace finances::investments::models;
using namespace finances::accounts::models;

namespace utils::db {

    namespace {
        std::vector<MovementNumerable> filter_by_fk_id(pqxx::work& tx, const std::string& fk_key,
                                                       const utils::db::Id& id) {
            auto query = std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name, "
                                     "mn.movement_ptr_id, mn.quantity, mn.unit_value, acc.id, acc.name, acc.ccy"
                                     " FROM {} AS mn"
                                     "   LEFT JOIN {} AS m ON mn.movement_ptr_id = m.id"
                                     "   LEFT JOIN {} AS t ON m.type_id = t.id"
                                     "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
                                     "   LEFT JOIN {} AS acc ON m.account_id = acc.id"
                                     " WHERE m.{} = $1"
                                     " ORDER BY m.date_value DESC",
                                     MOVEMENT_NUMERABLE_TABLE, MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE,
                                     ACCOUNT_TABLE, fk_key);
            SPDLOG_TRACE(query);

            std::vector<MovementNumerable> ret;
            for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name, mn_id,
                       quantity, unit_value, acc_id, acc_name, acc_ccy] :
                 tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id, std::string, Id,
                          Amount, Amount, Id, std::string, std::string>(query, pqxx::params{id})) {
                ret.emplace_back(MovementNumerable{
                    .movement = Movement{.id = id,
                                         .transaction = std::make_pair(transaction_id, transaction_name),
                                         .type = std::make_pair(type_id, type_name),
                                         .direction = direction,
                                         .account = std::make_pair(acc_id, acc_name),
                                         .date_value = date_value,
                                         .amount = Money{amount, Ccy{acc_ccy}}},
                    .id = mn_id,
                    .quantity = quantity,
                    .unit_value = Money{unit_value, Ccy{acc_ccy}},
                });
            }
            SPDLOG_TRACE("Found {} numerable movements", ret.size());
            return ret;
        }
    } // namespace

    template <>
    template <>
    std::vector<MovementNumerable>
    utils::db::ModelManager<MovementNumerable>::_filter_by_fk<Account>(pqxx::work& tx,
                                                                       const ModelData<Account>::Id& account_id) {
        SPDLOG_DEBUG("Get all numerable movements for account_id {}", account_id);
        return filter_by_fk_id(tx, "account_id", account_id);
    }

    template <>
    template <>
    std::vector<MovementNumerable> utils::db::ModelManager<MovementNumerable>::_filter_by_fk<Transaction>(
        pqxx::work& tx, const ModelData<Transaction>::Id& transaction_id) {
        SPDLOG_DEBUG("Get all numerable movements for transaction_id {}", transaction_id);
        return filter_by_fk_id(tx, "transaction_id", transaction_id);
    }

    template <>
    Id ModelManager<MovementNumerable>::_create(pqxx::work& tx, const MovementNumerable& movement_numerable) {
        SPDLOG_DEBUG("Create a new MovementNumerable");

        auto movement_id = ModelManager<Movement>::_create(tx, movement_numerable.movement);
        auto query = std::format("INSERT INTO {}"
                                 " (movement_ptr_id, quantity, unit_value)"
                                 " VALUES ($1, $2, $3)"
                                 " RETURNING movement_ptr_id;",
                                 MOVEMENT_NUMERABLE_TABLE);

        SPDLOG_TRACE(query);
        auto r =
            tx.exec(query, pqxx::params{movement_id, movement_numerable.quantity, movement_numerable.unit_value.amount})
                .one_field();
        auto inserted_id = r.as<Id>();
        return inserted_id;
    }
} // namespace utils::db

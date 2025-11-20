#include "movement.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

Money Movement::amount_in_local_ccy() const {
    if (fx) {
        const Fx fx_obj{.foreign = amount.ccy, .local = fx->second, .date_value = date_value, .rate = fx->first};

        return apply_fx(amount, fx_obj);
    } else {
        return amount;
    }
}
namespace utils::db {

    namespace {
        std::vector<Movement> filter_by_fk_id(pqxx::work& tx, const std::string& fk_key, const utils::db::Id& id) {
            auto query =
                std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name, "
                            "acc.id, acc.name, acc.ccy, fx.rate, fx.local"
                            " FROM {} AS m"
                            "   LEFT JOIN {} AS t ON m.type_id = t.id"
                            "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
                            "   LEFT JOIN {} AS acc ON m.account_id = acc.id"
                            "   LEFT JOIN {} AS fx ON m.fx_id = fx.id"
                            " WHERE m.{} = $1"
                            " ORDER BY m.date_value DESC",
                            MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE, ACCOUNT_TABLE, FX_TABLE, fk_key);
            SPDLOG_TRACE(query);
            std::vector<Movement> ret;
            for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name,
                       account_id, account_name, account_ccy, fx_rate, fx_local_ccy] :
                 tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id, std::string, Id,
                          std::string, std::string, std::optional<Amount>, std::optional<std::string>>(
                     query, pqxx::params{id})) {

                std::optional<std::pair<decltype(Fx::rate), decltype(Fx::local)>> fx = std::nullopt;
                if (fx_rate && fx_local_ccy) {
                    fx = std::make_pair(std::move(fx_rate.value()), Ccy{fx_local_ccy.value()});
                }

                ret.emplace_back(Movement{.id = id,
                                          .transaction = std::make_pair(transaction_id, transaction_name),
                                          .type = std::make_pair(type_id, type_name),
                                          .direction = direction,
                                          .account = std::make_pair(account_id, account_name),
                                          .date_value = date_value,
                                          .fx = fx,
                                          .amount = Money{amount, Ccy{account_ccy}}});
            }
            SPDLOG_TRACE("Found {} movements", ret.size());
            return ret;
        }
    } // namespace

    template <>
    template <>
    std::vector<Movement>
    utils::db::ModelManager<Movement>::_filter_by_fk<Account>(pqxx::work& tx,
                                                              const ModelData<Account>::Id& account_id) {
        SPDLOG_DEBUG("Get all movements for account_id {}", account_id);
        return filter_by_fk_id(tx, "account_id", account_id);
    }

    template <>
    template <>
    std::vector<Movement>
    utils::db::ModelManager<Movement>::_filter_by_fk<Transaction>(pqxx::work& tx,
                                                                  const ModelData<Transaction>::Id& transaction_id) {
        SPDLOG_DEBUG("Get all movements for transaction_id {}", transaction_id);
        return filter_by_fk_id(tx, "transaction_id", transaction_id);
    }

} // namespace utils::db

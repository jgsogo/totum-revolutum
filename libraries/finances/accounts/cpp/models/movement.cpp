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

        auto query =
            std::format("SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name, acc.name"
                        " FROM {} AS m"
                        "   LEFT JOIN {} AS t ON m.type_id = t.id"
                        "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
                        "   LEFT JOIN {} AS acc ON m.account_id = acc.id"
                        " WHERE m.account_id = $1"
                        " ORDER BY m.date_value DESC",
                        MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE, ACCOUNT_TABLE);
        SPDLOG_TRACE(query);
        std::vector<Movement> ret;
        for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name,
                   account_name] : tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id,
                                            std::string, std::string>(query, pqxx::params{account_id})) {
            ret.emplace_back(Movement{.id = id,
                                      .transaction = std::make_pair(transaction_id, transaction_name),
                                      .type = std::make_pair(type_id, type_name),
                                      .direction = direction,
                                      .account = std::make_pair(account_id, account_name),
                                      .date_value = date_value,
                                      .amount = amount});
        }
        SPDLOG_TRACE("Found {} movements for account {}", ret.size(), account_id);
        return ret;
    }

    template <>
    template <>
    std::vector<finances::accounts::models::Movement>
    utils::db::ModelManager<finances::accounts::models::Movement>::_filter_by_fk<
        finances::accounts::models::Transaction>(
        pqxx::work& tx, const ModelData<finances::accounts::models::Transaction>::Id& transaction_id) {
        SPDLOG_DEBUG("Get all movements for transaction_id {}", transaction_id);

        auto query = std::format(
            "SELECT m.id, m.date_value, m.amount, m.direction, t.id, t.name, tr.id, tr.name, acc.id, acc.name"
            " FROM {} AS m"
            "   LEFT JOIN {} AS t ON m.type_id = t.id"
            "   LEFT JOIN {} AS tr ON m.transaction_id = tr.id"
            "   LEFT JOIN {} AS acc ON m.account_id = acc.id"
            " WHERE m.transaction_id = $1"
            " ORDER BY m.date_value DESC",
            MOVEMENT_TABLE, MOVEMENTTYPE_TABLE, TRANSACTION_TABLE, ACCOUNT_TABLE);
        SPDLOG_TRACE(query);
        std::vector<Movement> ret;
        for (auto [id, date_value, amount, direction, type_id, type_name, transaction_id, transaction_name, account_id,
                   account_name] : tx.query<Id, utils::libpqxx::Date, Amount, MovementDirection, Id, std::string, Id,
                                            std::string, Id, std::string>(query, pqxx::params{transaction_id})) {
            ret.emplace_back(Movement{.id = id,
                                      .transaction = std::make_pair(transaction_id, transaction_name),
                                      .type = std::make_pair(type_id, type_name),
                                      .direction = direction,
                                      .account = std::make_pair(account_id, account_name),
                                      .date_value = date_value,
                                      .amount = amount});
        }
        SPDLOG_TRACE("Found {} movements for transaction {}", ret.size(), transaction_id);
        return ret;
    }

} // namespace utils::db

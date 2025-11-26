#include "transaction.h"

#include <spdlog/spdlog.h>

#include "model_manager.hpp"

using namespace finances::accounts::models;

namespace utils::db {

    template <> std::vector<Transaction> ModelManager<Transaction>::_all(pqxx::work& tx) {
        auto query = std::format("SELECT DISTINCT t.id, t.name, t.description, tg.id, tg.name"
                                 " FROM {} AS t"
                                 "   LEFT JOIN {} tg ON t.group_id = tg.id"
                                 "   JOIN {} AS m ON m.transaction_id = t.id;",
                                 TRANSACTION_TABLE, TRANSACTION_GROUP_TABLE, MOVEMENT_TABLE);
        SPDLOG_TRACE(query);

        std::vector<Transaction> ret;
        for (auto [id, name, description, tg_id, tg_name] :
             tx.query<Id, std::string, std::optional<std::string>, std::optional<Id>, std::optional<std::string>>(
                 query)) {

            std::optional<std::pair<decltype(TransactionGroup::id), decltype(TransactionGroup::name)>> group_data =
                std::nullopt;
            if (tg_id && tg_name) {
                group_data = std::make_pair(std::move(tg_id.value()), std::move(tg_name.value()));
            }

            ret.emplace_back(Transaction{.id = id, .name = name, .description = description, .group = group_data});
        }
        return {ret};
    };

    template <>
    ExpectedType<Transaction, ErrorNotFound, ErrorMultipleFound>
    ModelManager<Transaction>::_get(pqxx::work& tx, const decltype(Transaction::id)& transaction_id) {
        auto query = std::format("SELECT t.id, t.name, t.description, tg.id, tg.name"
                                 " FROM {} AS t"
                                 "   LEFT JOIN {} tg ON t.group_id = tg.id"
                                 " WHERE t.id = $1;",
                                 TRANSACTION_TABLE, TRANSACTION_GROUP_TABLE);
        SPDLOG_TRACE(query);

        auto r = tx.exec(query, pqxx::params{transaction_id}).one_row();
        auto [id, name, description, tg_id, tg_name] =
            r.as<Id, std::string, std::optional<std::string>, std::optional<Id>, std::optional<std::string>>();

        std::optional<std::pair<decltype(TransactionGroup::id), decltype(TransactionGroup::name)>> group_data =
            std::nullopt;
        if (tg_id && tg_name) {
            group_data = std::make_pair(std::move(tg_id.value()), std::move(tg_name.value()));
        }

        return {Transaction{.id = id, .name = name, .description = description, .group = group_data}};
    }

    template <>
    template <>
    std::vector<Transaction>
    utils::db::ModelManager<Transaction>::_filter_by_fk<Account>(pqxx::work& tx,
                                                                 const ModelData<Account>::Id& account_id) {

        auto query = std::format("SELECT DISTINCT t.id, t.name, t.description, tg.id, tg.name"
                                 " FROM {} AS t"
                                 "   LEFT JOIN {} tg ON t.group_id = tg.id"
                                 "   JOIN {} AS m ON m.transaction_id = t.id"
                                 " WHERE m.account_id = $1;",
                                 TRANSACTION_TABLE, TRANSACTION_GROUP_TABLE, MOVEMENT_TABLE);
        SPDLOG_TRACE(query);

        std::vector<Transaction> ret;
        for (auto [id, name, description, tg_id, tg_name] :
             tx.query<Id, std::string, std::optional<std::string>, std::optional<Id>, std::optional<std::string>>(
                 query, pqxx::params{account_id})) {

            std::optional<std::pair<decltype(TransactionGroup::id), decltype(TransactionGroup::name)>> group_data =
                std::nullopt;
            if (tg_id && tg_name) {
                group_data = std::make_pair(std::move(tg_id.value()), std::move(tg_name.value()));
            }

            ret.emplace_back(Transaction{.id = id, .name = name, .description = description, .group = group_data});
        }
        return {ret};
    }

    template <> Id ModelManager<Transaction>::_create(pqxx::work&, Transaction&&) {
        SPDLOG_ERROR("Not implemented");
        return {std::monostate{}};
    }

} // namespace utils::db

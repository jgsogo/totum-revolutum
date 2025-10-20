#include "transaction.h"

#include <spdlog/spdlog.h>

#include "model_manager.hpp"

using namespace finances::accounts::models;

namespace utils::db {

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

} // namespace utils::db

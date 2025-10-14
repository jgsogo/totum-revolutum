#include "account_holder.h"

#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/libpqxx/orm/model.h"

using namespace finances::accounts::models;

namespace utils::db {

    template <>
    ExpectedType<AccountHolder, ErrorNotFound, ErrorMultipleFound>
    ModelManager<AccountHolder>::_get(pqxx::work& tx, const ModelData<AccountHolder>::Id& id) {
        SPDLOG_DEBUG("Get account holder with pk {}", id);

        auto query = std::format(""
                                 "SELECT ah.id, ah.name, ah.is_company, ah.photo"
                                 " FROM {} AS ah"
                                 "      JOIN {} AS ahr ON ah.id = ahr.holder_id"
                                 " WHERE ah.id = $1;",
                                 ACCOUNT_HOLDER_TABLE, ACCOUNT_HOLDER_ROLE_TABLE);
        SPDLOG_TRACE(query);

        auto r = tx.exec(query, pqxx::params{id}).one_row();
        auto [_, name, is_company, photo, owns_money] = r.as<Id, std::string, bool, std::optional<std::string>, bool>();
        return {AccountHolder(
            {.id = id, .name = name, .is_company = is_company, .photo = photo, .owns_money = owns_money})};
    }

    template <> std::vector<AccountHolder> utils::db::ModelManager<AccountHolder>::_all(pqxx::work& tx) {
        SPDLOG_DEBUG("Get all account holders");

        auto query = std::format(""
                                 "SELECT ah.id, ah.name, ah.is_company, ah.photo, ahr.owns_money"
                                 " FROM {} AS ah"
                                 "      JOIN {} AS ahr ON ah.id = ahr.holder_id;",
                                 ACCOUNT_HOLDER_TABLE, ACCOUNT_HOLDER_ROLE_TABLE);
        SPDLOG_TRACE(query);

        std::vector<AccountHolder> ret;
        for (auto [id, name, is_company, photo, owns_money] :
             tx.query<Id, std::string, bool, std::optional<std::string>, bool>(query)) {
            ret.emplace_back(AccountHolder{
                .id = id, .name = name, .is_company = is_company, .photo = photo, .owns_money = owns_money});
        }
        SPDLOG_TRACE("Found {} account holders", ret.size());
        return ret;
    }

    template <>
    template <>
    std::vector<finances::accounts::models::AccountHolder>
    utils::db::ModelManager<finances::accounts::models::AccountHolder>::_filter_by_fk<
        finances::accounts::models::Account>(pqxx::work& tx,
                                             const decltype(finances::accounts::models::Account::id)& account_id) {
        SPDLOG_DEBUG("Get all account holders for account {}", account_id);

        auto query = std::format(""
                                 "SELECT ah.id, ah.name, ah.is_company, ah.photo, ahr.owns_money"
                                 " FROM {} AS ah"
                                 "      JOIN {} AS ahr ON ah.id = ahr.holder_id"
                                 " WHERE ahr.account_id = $1;",
                                 ACCOUNT_HOLDER_TABLE, ACCOUNT_HOLDER_ROLE_TABLE);
        SPDLOG_TRACE(query);

        std::vector<AccountHolder> ret;
        for (auto [id, name, is_company, photo, owns_money] :
             tx.query<Id, std::string, bool, std::optional<std::string>, bool>(query, pqxx::params{account_id})) {
            ret.emplace_back(AccountHolder{
                .id = id, .name = name, .is_company = is_company, .photo = photo, .owns_money = owns_money});
        }
        SPDLOG_TRACE("Found {} snapshots for account {}", ret.size(), account_id);
        return ret;
    }

} // namespace utils::db

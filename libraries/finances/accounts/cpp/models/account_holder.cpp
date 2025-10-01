#include "account_holder.h"

using namespace finances::accounts::models;

tl::expected<std::vector<std::pair<AccountHolder, AccountHolderRole>>, Error> AccountHolderManager::all(Id account_id) {
    return pool.with_conn<tl::expected<std::vector<std::pair<AccountHolder, AccountHolderRole>>, Error>>(
        [account_id](
            pqxx::connection& conn) -> tl::expected<std::vector<std::pair<AccountHolder, AccountHolderRole>>, Error> {
            try {
                pqxx::work tx(conn);
                SPDLOG_DEBUG("Get all account holders for account {}", account_id);

                // std::vector<Account> ret;
                auto query = std::format(""
                                         "SELECT ah.id, ah.name, ah.is_company, ah.photo, ahr.owns_money"
                                         "   FROM {} AS ah"
                                         "   JOIN {} AS ahr ON ah.id = ahr.holder_id"
                                         "   WHERE ahr.account_id = $1;",
                                         ACCOUNT_HOLDER_TABLE, ACCOUNT_HOLDER_ROLE_TABLE);
                SPDLOG_TRACE(query);

                std::vector<std::pair<AccountHolder, AccountHolderRole>> ret;
                for (auto [id, name, is_company, photo, owns_money] :
                     tx.query<Id, std::string, bool, std::optional<std::string>, bool>(query,
                                                                                       pqxx::params{account_id})) {
                    ret.emplace_back(
                        std::make_pair(AccountHolder{.id = id, .name = name, .is_company = is_company, .photo = photo},
                                       AccountHolderRole{.owns_money = owns_money}));
                }
                SPDLOG_TRACE("Found {} snapshots for account {}", ret.size(), account_id);
                return {ret};

            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch account holders for account {}: {}", account_id, e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

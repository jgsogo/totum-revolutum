#include "account.h"

#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

AccountManager::AccountManager(utils::db::ConnectionPool& pool) : ModelManager<Account>(pool) {}

template <> tl::expected<std::vector<Account>, Error> ModelManager<Account>::all() {
    return pool.with_conn<tl::expected<std::vector<Account>, Error>>([](pqxx::connection& conn)
                                                                         -> tl::expected<std::vector<Account>, Error> {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Get all accounts");

            std::vector<Account> ret;
            auto query = std::format(
                "SELECT a.id, a.name, a.description, a.identifier, a.ccy, t.id, t.name, c.id, c.name, a.is_numerable"
                " FROM {} a"
                "   LEFT JOIN {} t ON a.type_id = t.id"
                "   LEFT JOIN {} c ON a.custodian_id = c.id;",
                ACCOUNT_TABLE, ACCOUNT_TYPE_TABLE, CUSTODIAN_TABLE);
            SPDLOG_TRACE(query);
            for (auto [id, name, description, identifier, ccy, type_id, type_name, custodian_id, custodian_name,
                       is_numerable] :
                 tx.query<uint64_t, std::string, std::optional<std::string>, std::optional<std::string>, std::string,
                          uint64_t, std::string, uint64_t, std::string, bool>(query)) {
                ret.emplace_back(Account{.id = id,
                                         .name = name,
                                         .description = description,
                                         .identifier = identifier,
                                         .ccy = Ccy{std::move(ccy)},
                                         .type = std::make_pair(type_id, type_name),
                                         .custodian = std::make_pair(custodian_id, custodian_name),
                                         .is_numerable = is_numerable});
            }
            SPDLOG_TRACE("Found {} accounts", ret.size());
            return {ret};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to fetch all the accounts: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    });
}

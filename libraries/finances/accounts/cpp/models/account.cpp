#include "account.h"

#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

template <> tl::expected<std::vector<Account>, Error> AccountManager::all() {
    return pool.with_conn<tl::expected<std::vector<Account>, Error>>(
        [](pqxx::connection& conn) -> tl::expected<std::vector<Account>, Error> {
            try {
                pqxx::work tx(conn);
                SPDLOG_DEBUG("Get all accounts");

                std::vector<Account> ret;
                auto query = std::format(
                    "SELECT id, name, description, identifier, ccy, type_id, custodian_id, is_numerable FROM {};",
                    ACCOUNT_TABLE);
                SPDLOG_TRACE(query);
                for (auto [id, name, description, identifier, ccy, type_id, custodian_id, is_numerable] :
                     tx.query<uint64_t, std::string, std::optional<std::string>, std::optional<std::string>,
                              std::string, uint64_t, uint64_t, bool>(query)) {
                    ret.emplace_back(Account{.id = id,
                                             .name = name,
                                             .description = description,
                                             .identifier = identifier,
                                             .ccy = Ccy{std::move(ccy)},
                                             .type_id = type_id,
                                             .custodian_id = custodian_id,
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

#include "account.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

namespace utils::db {

    template <> std::vector<Account> utils::db::ModelManager<Account>::_all(pqxx::work& tx) {
        auto query = std::format("SELECT a.id, a.name, a.description, a.identifier, a.ccy, a.open, a.close, "
                                 "t.id, t.name, c.id, c.name, a.is_numerable"
                                 " FROM {} a"
                                 "   LEFT JOIN {} t ON a.type_id = t.id"
                                 "   LEFT JOIN {} c ON a.custodian_id = c.id;",
                                 ACCOUNT_TABLE, ACCOUNT_TYPE_TABLE, CUSTODIAN_TABLE);
        SPDLOG_TRACE(query);

        std::vector<Account> ret;
        for (auto [id, name, description, identifier, ccy, open, close, type_id, type_name, custodian_id,
                   custodian_name, is_numerable] :
             tx.query<Id, std::string, std::optional<std::string>, std::optional<std::string>, std::string,
                      utils::libpqxx::Date, std::optional<utils::libpqxx::Date>, Id, std::string, Id, std::string,
                      bool>(query)) {
            ret.emplace_back(Account{.id = id,
                                     .name = name,
                                     .description = description,
                                     .identifier = identifier,
                                     .ccy = Ccy{std::move(ccy)},
                                     .open = open,
                                     .close = close,
                                     .type = std::make_pair(type_id, type_name),
                                     .custodian = std::make_pair(custodian_id, custodian_name),
                                     .is_numerable = is_numerable});
        }
        return {ret};
    }

    template <>
    template <>
    std::vector<finances::accounts::models::Account>
    utils::db::ModelManager<finances::accounts::models::Account>::_filter_by_fk<Custodian>(
        pqxx::work& tx, const decltype(Custodian::id)& custodian_id) {
        auto query = std::format("SELECT a.id, a.name, a.description, a.identifier, a.ccy, a.open, a.close, "
                                 "t.id, t.name, c.id, c.name, a.is_numerable"
                                 " FROM {} a"
                                 "   LEFT JOIN {} t ON a.type_id = t.id"
                                 "   LEFT JOIN {} c ON a.custodian_id = c.id"
                                 " WEHRE a.custodian_id = $1;",
                                 ACCOUNT_TABLE, ACCOUNT_TYPE_TABLE, CUSTODIAN_TABLE);
        SPDLOG_TRACE(query);

        std::vector<Account> ret;
        for (auto [id, name, description, identifier, ccy, open, close, type_id, type_name, custodian_id,
                   custodian_name, is_numerable] :
             tx.query<Id, std::string, std::optional<std::string>, std::optional<std::string>, std::string,
                      utils::libpqxx::Date, std::optional<utils::libpqxx::Date>, Id, std::string, Id, std::string,
                      bool>(query, pqxx::params{custodian_id})) {
            ret.emplace_back(Account{.id = id,
                                     .name = name,
                                     .description = description,
                                     .identifier = identifier,
                                     .ccy = Ccy{std::move(ccy)},
                                     .open = open,
                                     .close = close,
                                     .type = std::make_pair(type_id, type_name),
                                     .custodian = std::make_pair(custodian_id, custodian_name),
                                     .is_numerable = is_numerable});
        }
        return {ret};
    }

    template <>
    template <>
    std::vector<finances::accounts::models::Account>
    utils::db::ModelManager<finances::accounts::models::Account>::_filter_by_fk<AccountType>(
        pqxx::work& tx, const decltype(AccountType::id)& type_id) {
        auto query = std::format("SELECT a.id, a.name, a.description, a.identifier, a.ccy, a.open, a.close, "
                                 "t.id, t.name, c.id, c.name, a.is_numerable"
                                 " FROM {} a"
                                 "   LEFT JOIN {} t ON a.type_id = t.id"
                                 "   LEFT JOIN {} c ON a.custodian_id = c.id"
                                 " WEHRE a.type_id = $1;",
                                 ACCOUNT_TABLE, ACCOUNT_TYPE_TABLE, CUSTODIAN_TABLE);
        SPDLOG_TRACE(query);

        std::vector<Account> ret;
        for (auto [id, name, description, identifier, ccy, open, close, type_id, type_name, custodian_id,
                   custodian_name, is_numerable] :
             tx.query<Id, std::string, std::optional<std::string>, std::optional<std::string>, std::string,
                      utils::libpqxx::Date, std::optional<utils::libpqxx::Date>, Id, std::string, Id, std::string,
                      bool>(query, pqxx::params{type_id})) {
            ret.emplace_back(Account{.id = id,
                                     .name = name,
                                     .description = description,
                                     .identifier = identifier,
                                     .ccy = Ccy{std::move(ccy)},
                                     .open = open,
                                     .close = close,
                                     .type = std::make_pair(type_id, type_name),
                                     .custodian = std::make_pair(custodian_id, custodian_name),
                                     .is_numerable = is_numerable});
        }
        return {ret};
    }

    template <>
    ExpectedType<Account, ErrorNotFound, ErrorMultipleFound>
    ModelManager<Account>::_get(pqxx::work& tx, const decltype(Account::id)& account_id) {
        auto query = std::format("SELECT a.id, a.name, a.description, a.identifier, a.ccy, a.open, a.close, "
                                 "t.id, t.name, c.id, c.name, a.is_numerable"
                                 " FROM {} a"
                                 "   LEFT JOIN {} t ON a.type_id = t.id"
                                 "   LEFT JOIN {} c ON a.custodian_id = c.id"
                                 " WHERE a.id = $1;",
                                 ACCOUNT_TABLE, ACCOUNT_TYPE_TABLE, CUSTODIAN_TABLE);
        SPDLOG_TRACE(query);

        auto r = tx.exec(query, pqxx::params{account_id}).one_row();
        auto [id, name, description, identifier, ccy, open, close, type_id, type_name, custodian_id,
                   custodian_name, is_numerable] = r.as<Id, std::string, std::optional<std::string>, std::optional<std::string>, std::string,
                      utils::libpqxx::Date, std::optional<utils::libpqxx::Date>, Id, std::string, Id, std::string,
                      bool>();
        return {Account{.id = id,
                                     .name = name,
                                     .description = description,
                                     .identifier = identifier,
                                     .ccy = Ccy{std::move(ccy)},
                                     .open = open,
                                     .close = close,
                                     .type = std::make_pair(type_id, type_name),
                                     .custodian = std::make_pair(custodian_id, custodian_name),
                                     .is_numerable = is_numerable}};

    }
} // namespace utils::db

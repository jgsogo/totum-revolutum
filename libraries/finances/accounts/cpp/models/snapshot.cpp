#include "snapshot.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/libpqxx/orm/model.h"

using namespace finances::accounts::models;

namespace utils::db {

    ExpectedType<std::optional<Snapshot>, DatabaseError>
    SnapshotManager::get_last_snapshot(const decltype(Account::id)& account_id) {
        return pool.with_conn<ExpectedType<std::optional<Snapshot>, DatabaseError>>(
            [&account_id](pqxx::connection& conn) -> ExpectedType<std::optional<Snapshot>, DatabaseError> {
                try {
                    pqxx::work tx(conn);
                    SPDLOG_DEBUG("Get last snapshot for account_id {}", account_id);

                    auto query = std::format("SELECT s.id, s.date_value, s.amount, acc.id, acc.name"
                                             " FROM {} AS s"
                                             "   LEFT JOIN {} AS acc ON s.account_id = acc.id"
                                             " WHERE s.account_id = $1"
                                             " ORDER BY s.date_value DESC"
                                             " LIMIT 1",
                                             SNAPSHOT_TABLE, ACCOUNT_TABLE);
                    SPDLOG_TRACE(query);
                    auto r = tx.exec(query, pqxx::params{account_id}).opt_row();
                    if (!r) {
                        return {std::nullopt};
                    }
                    auto [id, date_value, amount, acc_id, acc_name] =
                        r->as<Id, utils::libpqxx::Date, Amount, Id, std::string>();
                    return {std::make_optional<Snapshot>({.id = id,
                                                          .account = std::make_pair(acc_id, acc_name),
                                                          .date_value = date_value,
                                                          .amount = amount})};
                } catch (const std::exception& e) {
                    SPDLOG_ERROR("Failed to fetch latest snapshot for account {}: {}", account_id, e.what());
                    return tl::unexpected(DatabaseError{});
                }
            });
    }

    ExpectedType<std::vector<std::optional<Snapshot>>, DatabaseError>
    SnapshotManager::get_last_snapshots(const std::vector<decltype(Account::id)>& account_ids) {
        return pool.with_conn<ExpectedType<std::vector<std::optional<Snapshot>>, DatabaseError>>(
            [&account_ids](
                pqxx::connection& conn) -> ExpectedType<std::vector<std::optional<Snapshot>>, DatabaseError> {
                try {
                    pqxx::work tx(conn);
                    SPDLOG_DEBUG("Get last snapshot for account_ids {}", account_ids);

                    // FIXME: There has to be a better way to pass a std::vector to the placeholders
                    std::string subquery;
                    pqxx::params values;

                    if (!account_ids.empty()) {
                        std::ostringstream out;
                        pqxx::placeholders name;
                        for (auto it = account_ids.begin(); it != account_ids.end() - 1; it++) {
                            out << name.get() << ", ";
                            name.next();
                            values.append(*it);
                        }
                        out << name.get();
                        name.next();
                        values.append(account_ids.back());

                        subquery = out.str();
                    }

                    auto query = std::format(
                        "SELECT DISTINCT ON (s.account_id) s.id, s.account_id, s.date_value, s.amount, acc.id, acc.name"
                        " FROM {} AS s"
                        "   LEFT JOIN {} AS acc ON s.account_id = acc.id"
                        " WHERE s.account_id IN ({})"
                        " ORDER BY s.account_id, s.date_value DESC;",
                        SNAPSHOT_TABLE, ACCOUNT_TABLE, subquery);
                    SPDLOG_TRACE(query);

                    std::map<Id, Snapshot> snapshots;
                    for (auto [id, account_id, date_value, amount, acc_id, acc_name] :
                         tx.query<Id, Id, utils::libpqxx::Date, Amount, Id, std::string>(query, values)) {
                        SPDLOG_TRACE("Found snapshot for account {}: date {}, amount = {}", account_id, date_value,
                                     amount);
                        [[maybe_unused]] const auto [it, inserted] = snapshots.emplace(
                            std::make_pair(account_id, Snapshot{.id = id,
                                                                .account = std::make_pair(acc_id, acc_name),
                                                                .date_value = date_value,
                                                                .amount = amount}));
                        assert(inserted);
                    }

                    // Now we need to build the return vector with the same size as the input vector
                    std::vector<std::optional<Snapshot>> ret;
                    ret.resize(account_ids.size());
                    std::transform(account_ids.begin(), account_ids.end(), ret.begin(),
                                   [&snapshots](const Id& account_id) -> std::optional<Snapshot> {
                                       auto it = snapshots.find(account_id);
                                       return it == snapshots.end() ? std::nullopt : std::make_optional(it->second);
                                   });
                    return {ret};
                } catch (const std::exception& e) {
                    SPDLOG_ERROR("Failed to fetch latest snapshots for accounts {}: {}", account_ids, e.what());
                    return tl::unexpected(DatabaseError{});
                }
            });
    }

    template <>
    template <>
    std::vector<finances::accounts::models::Snapshot>
    utils::db::ModelManager<finances::accounts::models::Snapshot>::_filter_by_fk<finances::accounts::models::Account>(
        pqxx::work& tx, const ModelData<finances::accounts::models::Account>::Id& account_id) {

        SPDLOG_DEBUG("Get all snapshots for account_id {}", account_id);

        auto query = std::format("SELECT s.id, s.date_value, s.amount, acc.id, acc.name"
                                 " FROM {} AS s"
                                 "   LEFT JOIN {} AS acc ON s.account_id = acc.id"
                                 " WHERE s.account_id = $1"
                                 " ORDER BY s.date_value DESC",
                                 SNAPSHOT_TABLE, ACCOUNT_TABLE);
        SPDLOG_TRACE(query);
        std::vector<Snapshot> ret;
        for (auto [id, date_value, amount, acc_id, acc_name] :
             tx.query<Id, utils::libpqxx::Date, Amount, Id, std::string>(query, pqxx::params{account_id})) {
            ret.emplace_back(Snapshot{
                .id = id, .account = std::make_pair(acc_id, acc_name), .date_value = date_value, .amount = amount});
        }
        SPDLOG_TRACE("Found {} snapshots for account {}", ret.size(), account_id);
        return {ret};
    }

    template <>
    Id ModelManager<finances::accounts::models::Snapshot>::_create(pqxx::work&,
                                                                   finances::accounts::models::Snapshot&&) {
        SPDLOG_ERROR("Not implemented");
        return {std::monostate{}};
    }

} // namespace utils::db

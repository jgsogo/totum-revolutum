#include "snapshot.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

namespace utils::db {

    ExpectedType<std::optional<Snapshot>, DatabaseError>
    SnapshotManager::get_last_snapshot(const decltype(Account::id)& account_id) {
        return tl::unexpected{NotImplemented{"TODO"}};
    }

} // namespace utils::db

tl::expected<std::optional<Snapshot>, Error>
SnapshotManager::get_last_snapshot(decltype(Account::id) account_id) const {
    return pool.with_conn<tl::expected<std::optional<Snapshot>, Error>>(
        [account_id](pqxx::connection& conn) -> tl::expected<std::optional<Snapshot>, Error> {
            try {
                pqxx::work tx(conn);
                SPDLOG_DEBUG("Get last snapshot for account_id {}", account_id);

                auto query = std::format("SELECT id, date_value, amount"
                                         " FROM {}"
                                         " WHERE account_id = $1"
                                         " ORDER BY date_value DESC"
                                         " LIMIT 1",
                                         SNAPSHOT_TABLE);
                SPDLOG_TRACE(query);
                auto r = tx.exec(query, pqxx::params{account_id}).opt_row();
                if (!r) {
                    return {std::nullopt};
                }
                auto [id, date_value, amount] = r->as<Id, utils::libpqxx::Date, Amount>();
                return {std::make_optional<Snapshot>(
                    {.id = id, .account_id = account_id, .date_value = date_value, .amount = amount})};
            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch latest snapshot for account {}: {}", account_id, e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

tl::expected<std::vector<std::optional<Snapshot>>, Error>
SnapshotManager::get_last_snapshots(const std::vector<decltype(Account::id)>& account_ids) const {
    return pool.with_conn<tl::expected<std::vector<std::optional<Snapshot>>, Error>>(
        [account_ids](pqxx::connection& conn) -> tl::expected<std::vector<std::optional<Snapshot>>, Error> {
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

                auto query = std::format("SELECT DISTINCT ON (account_id) id, account_id, date_value, amount"
                                         " FROM {}"
                                         " WHERE account_id IN ({})"
                                         " ORDER BY account_id, date_value DESC;",
                                         SNAPSHOT_TABLE, subquery);
                SPDLOG_TRACE(query);

                std::map<Id, Snapshot> snapshots;
                for (auto [id, account_id, date_value, amount] :
                     tx.query<Id, Id, utils::libpqxx::Date, Amount>(query, values)) {
                    SPDLOG_TRACE("Found snapshot for account {}: date {}, amount = {}", account_id, date_value, amount);
                    [[maybe_unused]] const auto [it, inserted] = snapshots.emplace(std::make_pair(
                        account_id,
                        Snapshot{.id = id, .account_id = account_id, .date_value = date_value, .amount = amount}));
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
                return tl::unexpected(Error::DBError);
            }
        });
}

tl::expected<std::vector<Snapshot>, Error> SnapshotManager::all(Id account_id) {
    return pool.with_conn<tl::expected<std::vector<Snapshot>, Error>>(
        [account_id](pqxx::connection& conn) -> tl::expected<std::vector<Snapshot>, Error> {
            try {
                pqxx::work tx(conn);
                SPDLOG_DEBUG("Get all snapshots for account_id {}", account_id);

                auto query = std::format("SELECT id, date_value, amount"
                                         " FROM {}"
                                         " WHERE account_id = $1"
                                         " ORDER BY date_value DESC",
                                         SNAPSHOT_TABLE);
                SPDLOG_TRACE(query);
                std::vector<Snapshot> ret;
                for (auto [id, date_value, amount] :
                     tx.query<Id, utils::libpqxx::Date, Amount>(query, pqxx::params{account_id})) {
                    ret.emplace_back(
                        Snapshot{.id = id, .account_id = account_id, .date_value = date_value, .amount = amount});
                }
                SPDLOG_TRACE("Found {} snapshots for account {}", ret.size(), account_id);
                return {ret};
            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch all snapshots for account {}: {}", account_id, e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

tl::expected<void, Error> SnapshotManager::create(Id account_id, utils::libpqxx::Date&& date_value, Amount&& amount) {
    return pool.with_conn<tl::expected<void, Error>>([&](pqxx::connection& conn) -> tl::expected<void, Error> {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Add snapshot to account_id {}: date_value={}, amount={}", account_id, date_value, amount);

            auto query = std::format("INSERT INTO {} (account_id, date_value, amount)"
                                     " VALUES ($1, $2, $3)",
                                     SNAPSHOT_TABLE);
            SPDLOG_TRACE(query);
            tx.exec(query, pqxx::params{account_id, date_value, amount}).no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert snapshots for account {}: {}", account_id, e.what());
            return tl::unexpected(Error::DBError);
        }
    });
}

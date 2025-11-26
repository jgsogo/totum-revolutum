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

                    auto query = std::format("SELECT s.id, s.date_value, s.amount, acc.id, acc.name, acc.ccy"
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
                    auto [id, date_value, amount, acc_id, acc_name, acc_ccy] =
                        r->as<Id, utils::libpqxx::Date, Amount, Id, std::string, std::string>();
                    return {std::make_optional<Snapshot>({.id = id,
                                                          .account = std::make_pair(acc_id, acc_name),
                                                          .date_value = date_value,
                                                          .amount = Money{amount, Ccy{acc_ccy}}})};
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

                    auto query = std::format("SELECT DISTINCT ON (s.account_id) s.id, s.account_id, s.date_value, "
                                             "s.amount, acc.id, acc.name, acc.ccy"
                                             " FROM {} AS s"
                                             "   LEFT JOIN {} AS acc ON s.account_id = acc.id"
                                             " WHERE s.account_id IN ({})"
                                             " ORDER BY s.account_id, s.date_value DESC;",
                                             SNAPSHOT_TABLE, ACCOUNT_TABLE, subquery);
                    SPDLOG_TRACE(query);

                    std::map<Id, Snapshot> snapshots;
                    for (auto [id, account_id, date_value, amount, acc_id, acc_name, acc_ccy] :
                         tx.query<Id, Id, utils::libpqxx::Date, Amount, Id, std::string, std::string>(query, values)) {
                        SPDLOG_TRACE("Found snapshot for account {}: date {}, amount = {}", account_id, date_value,
                                     amount);
                        [[maybe_unused]] const auto [it, inserted] = snapshots.emplace(
                            std::make_pair(account_id, Snapshot{.id = id,
                                                                .account = std::make_pair(acc_id, acc_name),
                                                                .date_value = date_value,
                                                                .amount = Money{amount, Ccy{acc_ccy}}}));
                        assert(inserted);
                    }

                    // Now we need to build the return vector with the same size as the input vector
                    std::vector<std::optional<Snapshot>> ret;
                    ret.resize(account_ids.size());
                    std::transform(account_ids.begin(), account_ids.end(), ret.begin(),
                                   [&snapshots](const Id& account_id) -> std::optional<Snapshot> {
                                       auto it = snapshots.find(account_id);
                                       return it == snapshots.end() ? std::nullopt
                                                                    : std::make_optional(std::move(it->second));
                                   });
                    return ExpectedType<std::vector<std::optional<Snapshot>>, DatabaseError>{std::move(ret)};
                } catch (const std::exception& e) {
                    SPDLOG_ERROR("Failed to fetch latest snapshots for accounts {}: {}", account_ids, e.what());
                    return tl::unexpected(DatabaseError{});
                }
            });
    }

    template <>
    template <>
    std::vector<Snapshot>
    utils::db::ModelManager<Snapshot>::_filter_by_fk<Account>(pqxx::work& tx,
                                                              const ModelData<Account>::Id& account_id) {

        SPDLOG_DEBUG("Get all snapshots for account_id {}", account_id);

        auto query = std::format("SELECT s.id, s.date_value, s.amount, acc.id, acc.name, acc.ccy"
                                 " FROM {} AS s"
                                 "   LEFT JOIN {} AS acc ON s.account_id = acc.id"
                                 " WHERE s.account_id = $1"
                                 " ORDER BY s.date_value DESC",
                                 SNAPSHOT_TABLE, ACCOUNT_TABLE);
        SPDLOG_TRACE(query);
        std::vector<Snapshot> ret;
        for (auto [id, date_value, amount, acc_id, acc_name, acc_ccy] :
             tx.query<Id, utils::libpqxx::Date, Amount, Id, std::string, std::string>(query,
                                                                                      pqxx::params{account_id})) {
            ret.emplace_back(Snapshot{.id = id,
                                      .account = std::make_pair(acc_id, acc_name),
                                      .date_value = date_value,
                                      .amount = Money{amount, Ccy{acc_ccy}}});
        }
        SPDLOG_TRACE("Found {} snapshots for account {}", ret.size(), account_id);
        return ret;
    }

    template <> Id ModelManager<Snapshot>::_create(pqxx::work& tx, Snapshot&& snapshot) {
        SPDLOG_DEBUG("Create a new snapshot");

        auto query = std::format(""
                                 "INSERT INTO {}"
                                 " (amount, account_id, date_value)"
                                 " VALUES ($1, $2, $3)"
                                 " RETURNING id;",
                                 SNAPSHOT_TABLE);

        SPDLOG_TRACE(query);
        auto r = tx.exec(query, pqxx::params{snapshot.amount.amount, snapshot.account.first, snapshot.date_value})
                     .one_field();
        auto snapshot_id = r.as<Id>();
        return snapshot_id;
    }

    template <> std::vector<Snapshot> ModelManager<Snapshot>::_all(pqxx::work&) {
        SPDLOG_ERROR("Not implemented");
        return {};
    }

    template <>
    ExpectedType<Snapshot, ErrorNotFound, ErrorMultipleFound>
    ModelManager<Snapshot>::_get(pqxx::work&, const decltype(Snapshot::id)&) {
        SPDLOG_ERROR("Not implemented");
        return tl::unexpected{NotImplemented{}};
    }

} // namespace utils::db

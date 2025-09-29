#include "snapshot_numerable.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

#include "constants.h"

using namespace finances::investments::models;
using namespace finances::accounts::models;

tl::expected<std::vector<SnapshotNumerable>, finances::accounts::models::Error>
SnapshotNumerableManager::all(finances::accounts::models::Id account_id) {
    return pool.with_conn<tl::expected<std::vector<SnapshotNumerable>, Error>>(
        [account_id](pqxx::connection& conn) -> tl::expected<std::vector<SnapshotNumerable>, Error> {
            try {
                pqxx::work tx(conn);
                SPDLOG_DEBUG("Get all snapshots for account_id {}", account_id);

                auto query =
                    std::format("SELECT s.id, s.date_value, s.amount, n.snapshot_ptr_id, n.quantity, n.unit_value"
                                " FROM {} AS s"
                                "   LEFT JOIN {} AS n ON n.snapshot_ptr_id = s.id"
                                " WHERE s.account_id = $1"
                                " ORDER BY s.date_value DESC",
                                SNAPSHOT_TABLE, SNAPSHOT_NUMERABLE_TABLE);
                SPDLOG_TRACE(query);
                std::vector<SnapshotNumerable> ret;
                for (auto [id, date_value, amount, numerable_id, quantity, unit_value] :
                     tx.query<Id, utils::libpqxx::Date, Amount, Id, Amount, Amount>(query, pqxx::params{account_id})) {
                    ret.emplace_back(SnapshotNumerable{
                        .snapshot =
                            Snapshot{.id = id, .account_id = account_id, .date_value = date_value, .amount = amount},
                        .id = numerable_id,
                        .quantity = quantity,
                        .unit_value = unit_value,
                    });
                }
                SPDLOG_TRACE("Found {} snapshots for account {}", ret.size(), account_id);
                return {ret};
            } catch (const std::exception& e) {
                SPDLOG_ERROR("Failed to fetch all snapshots for account {}: {}", account_id, e.what());
                return tl::unexpected(Error::DBError);
            }
        });
}

tl::expected<void, finances::accounts::models::Error>
SnapshotNumerableManager::create(finances::accounts::models::Id account_id, utils::libpqxx::Date&& date_value,
                                 finances::accounts::models::Amount&& quantity,
                                 finances::accounts::models::Amount&& unit_value) {
    SPDLOG_ERROR("Not implemented");
    return tl::unexpected{finances::accounts::models::Error::NotImplemented};
}

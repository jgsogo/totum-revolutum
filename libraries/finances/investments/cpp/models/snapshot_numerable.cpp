#include "snapshot_numerable.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

#include "constants.h"
#include "types/numerable_amount.h"

using namespace finances::investments::models;
using namespace finances::accounts::models;

namespace utils::db {

    template <>
    template <>
    std::vector<finances::investments::models::SnapshotNumerable>
    utils::db::ModelManager<finances::investments::models::SnapshotNumerable>::_filter_by_fk<
        finances::accounts::models::Account>(pqxx::work& tx,
                                             const ModelData<finances::accounts::models::Account>::Id& account_id) {
        SPDLOG_DEBUG("Get all snapshots for account_id {}", account_id);

        auto query = std::format("SELECT s.id, s.date_value, s.amount, n.snapshot_ptr_id, n.quantity, n.unit_value, "
                                 "acc.id, acc.name, acc.ccy"
                                 " FROM {} AS s"
                                 "   LEFT JOIN {} AS n ON n.snapshot_ptr_id = s.id"
                                 "   LEFT JOIN {} AS acc ON s.account_id = acc.id"
                                 " WHERE s.account_id = $1"
                                 " ORDER BY s.date_value DESC",
                                 SNAPSHOT_TABLE, SNAPSHOT_NUMERABLE_TABLE, ACCOUNT_TABLE);
        SPDLOG_TRACE(query);
        std::vector<SnapshotNumerable> ret;
        for (auto [id, date_value, amount, numerable_id, quantity, unit_value, acc_id, acc_name, acc_ccy] :
             tx.query<Id, utils::libpqxx::Date, Amount, Id, Amount, Amount, Id, std::string, std::string>(
                 query, pqxx::params{account_id})) {
            ret.emplace_back(SnapshotNumerable{
                .snapshot = Snapshot{.id = id,
                                     .account = std::make_pair(acc_id, acc_name),
                                     .date_value = date_value,
                                     .amount = Money{amount, Ccy{acc_ccy}}},
                .id = numerable_id,
                .quantity = quantity,
                .unit_value = Money{unit_value, Ccy{acc_ccy}},
            });
        }
        SPDLOG_TRACE("Found {} snapshots for account {}", ret.size(), account_id);
        return ret;
    }

    template <>
    Id ModelManager<finances::investments::models::SnapshotNumerable>::_create(
        pqxx::work& tx, finances::investments::models::SnapshotNumerable&& snapshot_numerable) {
        SPDLOG_DEBUG("Create a new snapshot numerable");
        auto query = std::format("WITH inserted_parent AS ("
                                 "    INSERT INTO {} (amount, account_id, date_value)"
                                 "    VALUES ($1, $2, $3)"
                                 "    RETURNING id"
                                 ") "
                                 "INSERT INTO {}"
                                 " (snapshot_ptr_id, quantity, unit_value)"
                                 " SELECT id, $4, $5"
                                 "   FROM inserted_parent"
                                 " RETURNING snapshot_ptr_id;",
                                 SNAPSHOT_TABLE, SNAPSHOT_NUMERABLE_TABLE);

        SPDLOG_TRACE(query);
        auto r = tx.exec(query,
                         pqxx::params{snapshot_numerable.snapshot.amount.amount,
                                      snapshot_numerable.snapshot.account.first, snapshot_numerable.snapshot.date_value,
                                      snapshot_numerable.quantity, snapshot_numerable.unit_value.amount})
                     .one_field();
        auto snapshot_id = r.as<Id>();
        return snapshot_id;
    }

    template <>
    std::vector<finances::investments::models::SnapshotNumerable>
    ModelManager<finances::investments::models::SnapshotNumerable>::_all(pqxx::work&) {
        SPDLOG_ERROR("Not implemented");
        return {};
    }

    template <>
    ExpectedType<finances::investments::models::SnapshotNumerable, ErrorNotFound, ErrorMultipleFound>
    ModelManager<finances::investments::models::SnapshotNumerable>::_get(
        pqxx::work&, const decltype(finances::investments::models::SnapshotNumerable::id)&) {
        SPDLOG_ERROR("Not implemented");
        return tl::unexpected{NotImplemented{}};
    }
} // namespace utils::db

#include "model.h"

using namespace finances::accounts::models;

namespace utils::db {

    template <>
    ExpectedType<AccountDetail, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<AccountDetail>::get(const ModelData<AccountDetail>::Id& account_id) {
        // Get all movements
        utils::db::ModelManager<Movement> mov_manager{pool};
        auto all_movs = mov_manager.filter_by_fk<Account>(account_id);
        if (!all_movs) {
            return ExpectedType<AccountDetail, DatabaseError, ErrorNotFound, ErrorMultipleFound>(
                std::move(all_movs.error()));
        }

        // Get all snapshots
        SnapshotManager snapshot_manager{pool};
        auto all_snapshots = snapshot_manager.filter_by_fk<Account>(account_id);
        if (!all_snapshots) {
            return ExpectedType<AccountDetail, DatabaseError, ErrorNotFound, ErrorMultipleFound>(
                std::move(all_snapshots.error()));
        }

        return {AccountDetail{
            .id = account_id,
            .movements = std::move(all_movs.value()),
            .snapshots = std::move(all_snapshots.value()),
        }};
    }

} // namespace utils::db

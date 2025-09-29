#include "snapshot_numerable.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

using namespace finances::investments::models;

tl::expected<std::vector<SnapshotNumerable>, finances::accounts::models::Error>
SnapshotNumerableManager::all(finances::accounts::models::Id account_id) {
    SPDLOG_ERROR("Not implemented");
    return tl::unexpected{finances::accounts::models::Error::NotImplemented};
}

tl::expected<void, finances::accounts::models::Error>
SnapshotNumerableManager::create(finances::accounts::models::Id account_id, utils::libpqxx::Date&& date_value,
                                 finances::accounts::models::Amount&& quantity,
                                 finances::accounts::models::Amount&& unit_value) {
    SPDLOG_ERROR("Not implemented");
    return tl::unexpected{finances::accounts::models::Error::NotImplemented};
}

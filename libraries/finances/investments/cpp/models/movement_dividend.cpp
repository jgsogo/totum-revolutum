#include "movement_dividend.h"

#include <spdlog/fmt/ranges.h>
#include <spdlog/spdlog.h>

using namespace finances::investments::models;

tl::expected<std::vector<MovementDividend>, finances::accounts::models::Error>
MovementDividendManager::all(finances::accounts::models::Id account_id) {
    SPDLOG_ERROR("Not implemented");
    return tl::unexpected{finances::accounts::models::Error::NotImplemented};
}

#include "model.h"

using namespace finances::accounts::models;

namespace utils::db {

    template <> ExpectedType<std::vector<MovementModel>, DatabaseError> ModelManager<MovementModel>::all() {
        // Get all movements
        auto movs_manager = ModelData<Movement>::Manager{pool};
        auto all = movs_manager.all();
        if (!all) {
            return tl::unexpected{all.error()};
        }

        std::vector<MovementModel> ret;
        for (auto&& item : all.value()) {

            // Get breadcrumb
            AccountTypeManager movtype_manager{pool};
            auto breadcrumb = movtype_manager.breadcrumb(item.type.first);

            std::string breadcrumb_str;
            if (!breadcrumb) {
                SPDLOG_WARN("Error retrieving movtype breadcrumb for movement {}: {}", item.id, breadcrumb.error());
                // TODO: Communicate error to user
                breadcrumb_str = item.type.second;
            } else {
                for (auto it : breadcrumb.value()) {
                    breadcrumb_str.append(it.second);
                    breadcrumb_str.append(" > ");
                }
                breadcrumb_str.append(item.type.second);
            }

            // Create the MovementModel, we have all the information we need
            ret.emplace_back(MovementModel{
                .id = item.id, .movement = std::move(item), .movtype_breadcrumb = std::move(breadcrumb_str)});
        }

        return ret;
    }

} // namespace utils::db

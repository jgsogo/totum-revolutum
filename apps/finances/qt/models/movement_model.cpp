#include "movement_model.h"

#include <spdlog/spdlog.h>

using namespace finances::accounts::models;
using namespace finances::investments::models;

const utils::libpqxx::Date& MovementModel::date_value() const {
    return std::visit(
        [](const auto& arg) -> const utils::libpqxx::Date& {
            using T = std::decay_t<decltype(arg)>;
            if constexpr (std::is_same_v<T, finances::accounts::models::Movement>) {
                return arg.date_value;
            } else if constexpr (std::is_same_v<T, finances::investments::models::MovementNumerable>) {
                return arg.movement.date_value;
            } else if constexpr (std::is_same_v<T, finances::investments::models::MovementDividend>) {
                return arg.movement.date_value;
            } else {
                static_assert(false, "non-exhaustive visitor!");
            }
        },
        movement);
}

namespace utils::db {
    template <>
    template <>
    ExpectedType<std::vector<MovementModel>, DatabaseError>
    ModelManager<MovementModel>::filter_by_fk<finances::accounts::models::Account>(
        const finances::accounts::models::Account& account) {
        // FIXME: If we use AccountModel instead, we know if it is numerable or not and we can
        //        choose which movements to retrieve.
        //
        //        But, I'm not sure if that's what we want:
        //          Q: Are there "numerable" accounts where we have regular movements?
        //          Q: Are there non-numerable accounts with dividends, or other investment movements?

        // All MovementNumerable
        auto movs_numerable_manager = ModelData<MovementNumerable>::Manager{pool};
        auto all_movs_numerable = movs_numerable_manager.filter_by_fk(account);
        if (!all_movs_numerable) {
            return tl::unexpected{all_movs_numerable.error()};
        }

        // All MovementDividend
        auto movs_dividend_manager = ModelData<MovementDividend>::Manager{pool};
        auto all_movs_dividend = movs_dividend_manager.filter_by_fk(account);
        if (!all_movs_dividend) {
            return tl::unexpected{all_movs_dividend.error()};
        }

        // All regular Movements
        auto movs_manager = ModelData<Movement>::Manager{pool};
        auto all_movs = movs_manager.filter_by_fk(account);
        if (!all_movs) {
            return tl::unexpected{all_movs.error()};
        }
        // - remove movements that are already dividend or numerable ones
        std::erase_if(all_movs.value(), [&all_movs_numerable, &all_movs_dividend](const auto& mov) {
            const auto& mov_id = mov.id;
            return ((std::find_if(all_movs_numerable->begin(), all_movs_numerable->end(),
                                  [&mov_id](const auto& item) { return item.movement.id == mov_id; }) !=
                     all_movs_numerable->end()) ||
                    (std::find_if(all_movs_dividend->begin(), all_movs_dividend->end(), [&mov_id](const auto& item) {
                         return item.movement.id == mov_id;
                     }) != all_movs_dividend->end()));
        });

        // A function to get the breadcrumb
        MovementTypeManager movtype_manager{pool};
        auto get_breadcrumb = [&movtype_manager](const Movement& item) -> std::string {
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
            return breadcrumb_str;
        };

        std::vector<MovementModel> ret;

        // Add regular Movement
        for (auto&& item : all_movs.value()) {
            ret.emplace_back(
                MovementModel{.id = item.id, .movement = std::move(item), .movtype_breadcrumb = get_breadcrumb(item)});
        }

        // Add MovementDividend
        for (auto&& item : all_movs_dividend.value()) {
            ret.emplace_back(MovementModel{
                .id = item.id, .movement = std::move(item), .movtype_breadcrumb = get_breadcrumb(item.movement)});
        }

        // Add MovementNumerable
        for (auto&& item : all_movs_numerable.value()) {
            ret.emplace_back(MovementModel{
                .id = item.id, .movement = std::move(item), .movtype_breadcrumb = get_breadcrumb(item.movement)});
        }

        return ret;
    }

    template <> ExpectedType<std::vector<MovementModel>, DatabaseError> ModelManager<MovementModel>::all() {
        SPDLOG_ERROR("Not implemented");
        return {};
    }

    template <>
    ExpectedType<MovementModel, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<MovementModel>::get(const ModelData<MovementModel>::Id&) {
        SPDLOG_ERROR("Not implemented");
        return tl::unexpected{NotImplemented{}};
    }

} // namespace utils::db

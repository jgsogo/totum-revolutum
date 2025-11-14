#include "movtype_model.h"

#include <spdlog/spdlog.h>

using namespace finances::accounts::models;

namespace utils::db {

    namespace {
        ExpectedType<std::string, DatabaseError> get_breadcrumb(MovementTypeManager& manager, MovementType& movtype) {
            auto breadcrumb_expected = manager.breadcrumb(movtype.id);
            if (!breadcrumb_expected) {
                return tl::unexpected{breadcrumb_expected.error()};
            }

            std::string breadcrumb_str;
            for (auto it : breadcrumb_expected.value()) {
                breadcrumb_str.append(it.second);
                breadcrumb_str.append(" > ");
            }
            breadcrumb_str.append(movtype.name);

            return breadcrumb_str;
        }
    } // namespace

    template <> ExpectedType<std::vector<MovementTypeModel>, DatabaseError> ModelManager<MovementTypeModel>::all() {

        // Get all movtypes
        auto manager = MovementTypeManager{pool};
        auto all_mov_types = manager.all();
        if (!all_mov_types) {
            return tl::unexpected{all_mov_types.error()};
        }

        std::vector<MovementTypeModel> ret;
        for (auto&& movtype : all_mov_types.value()) {
            auto breadcrumb_expected = get_breadcrumb(manager, movtype);
            if (!breadcrumb_expected) {
                return tl::unexpected{breadcrumb_expected.error()};
            }
            ret.emplace_back(MovementTypeModel{
                .id = movtype.id, .movtype = std::move(movtype), .breadcrumb = std::move(breadcrumb_expected.value())});
        }

        return ret;
    }

    template <>
    ExpectedType<MovementTypeModel, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<MovementTypeModel>::get(const ModelData<MovementTypeModel>::Id& id) {

        auto manager = MovementTypeManager{pool};
        auto movtype_expected = manager.get(id);
        if (!movtype_expected) {
            return tl::unexpected{movtype_expected.error()};
        }

        finances::accounts::models::MovementType movtype = std::move(movtype_expected.value());
        auto breadcrumb_expected = get_breadcrumb(manager, movtype);
        if (!breadcrumb_expected) {
            return tl::unexpected{movtype_expected.error()};
        }

        return MovementTypeModel{
            .id = movtype.id, .movtype = std::move(movtype), .breadcrumb = std::move(breadcrumb_expected.value())};
    }
} // namespace utils::db

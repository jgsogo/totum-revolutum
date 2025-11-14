#include "hierarchy_tree_columns.h"

#include <QBrush>
#include <QColor>
#include <QDate>
#include <QFont>

// #include "apps/finances/qt/metatypes/types.h"
// #include "apps/finances/qt/utils/utils.h"

using namespace finances::accounts::models;

namespace utils::qt::models {

    template <>
    QVariant
    DataDispatcher<MovementTypeModel, HierarchyTreeColumns, Qt::DisplayRole>::data(const MovementTypeModel& item,
                                                                                   HierarchyTreeColumns column) {
        QVariant result;

        const MovementType& movtype = item.movtype;

        switch (column) {
        case HierarchyTreeColumns::ID:
            result.setValue(item.id);
            break;
        case HierarchyTreeColumns::NAME:
            result = movtype.name.c_str();
            break;
        case HierarchyTreeColumns::BREADCRUMB:
            result = item.breadcrumb.c_str();
            break;
        case HierarchyTreeColumns::DESCRIPTION:
            if (movtype.description) {
                result = movtype.description.value().c_str();
            }
            break;
        case HierarchyTreeColumns::IS_ABSTRACT:
            result = movtype.is_abstract;
            break;
        }

        return result;
    }

} // namespace utils::qt::models

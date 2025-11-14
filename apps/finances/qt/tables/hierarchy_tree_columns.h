#pragma once

#include "libraries/utils/cpp/qt/models/data_dispatcher.h"

#include "apps/finances/qt/models/movtype_model.h"

enum class HierarchyTreeColumns {
    ID = 0,
    NAME = 1,
    BREADCRUMB = 2,
    DESCRIPTION = 3,
    IS_ABSTRACT = 4,
};

namespace utils::qt::models {

    template <>
    QVariant DataDispatcher<MovementTypeModel, HierarchyTreeColumns, Qt::DisplayRole>::data(const MovementTypeModel&,
                                                                                            HierarchyTreeColumns);

} // namespace utils::qt::models

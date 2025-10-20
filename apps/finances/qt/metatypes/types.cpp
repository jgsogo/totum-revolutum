#include "types.h"

#include <QString>

void register_metatypes() {
    QMetaType::registerConverter<utils::db::Id, QString>(
        [](const utils::db::Id& id) { return QString::fromStdString(std::format("{}", id)); });
}

#include "types.h"

#include <QString>

void register_metatypes() {
    QMetaType::registerConverter<utils::db::Id, QString>(
        [](const utils::db::Id& id) { return QString::fromStdString(std::format("{}", id)); });

    QMetaType::registerConverter<finances::accounts::models::Money, QString>(
        [](const finances::accounts::models::Money& money) {
            return QString::fromStdString(static_cast<std::string>(money));
        });
}

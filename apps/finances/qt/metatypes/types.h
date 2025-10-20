#pragma once

#include <QMetaType>

#include "libraries/utils/cpp/libpqxx/orm/id.h"

Q_DECLARE_METATYPE(utils::db::Id)

void register_metatypes();

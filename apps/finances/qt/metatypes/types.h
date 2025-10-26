#pragma once

#include <QMetaType>

#include "libraries/finances/accounts/cpp/models/types/money.h"
#include "libraries/utils/cpp/libpqxx/orm/id.h"

Q_DECLARE_METATYPE(utils::db::Id);
Q_DECLARE_METATYPE(finances::accounts::models::Money);

void register_metatypes();

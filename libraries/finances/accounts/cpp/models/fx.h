#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"

#include "types/amount.h"
#include "types/ccy.h"
#include "types/money.h"

namespace finances::accounts::models {

    struct Fx {
        Ccy foreign;
        Ccy local;

        utils::libpqxx::Date date_value;
        Amount rate;
    };

    Money apply_fx(const Money&, const Fx&);

} // namespace finances::accounts::models

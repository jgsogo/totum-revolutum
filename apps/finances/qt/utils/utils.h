#pragma once

#include <QDate>
#include <QString>
#include <tl/expected.hpp>

#include "libraries/finances/accounts/cpp/models/types/amount.h"
#include "libraries/utils/cpp/libpqxx/datatypes/date.h"

namespace utils {
    tl::expected<finances::accounts::models::Amount, std::string> qstring_to_amount(QString&& input);

    QDate date_to_qdate(const utils::libpqxx::Date& date);
} // namespace utils

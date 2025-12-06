#pragma once

#include <QDate>
#include <QString>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"

#include "libraries/finances/accounts/cpp/models/types/amount.h"
#include "libraries/finances/accounts/cpp/models/types/ccy.h"
namespace utils {
    ExpectedType<finances::accounts::models::Amount, utils::libpqxx::ParseNumericError>
    qstring_to_amount(const QString& input, const finances::accounts::models::Ccy& ccy);

    QDate date_to_qdate(const utils::libpqxx::Date& date);
} // namespace utils

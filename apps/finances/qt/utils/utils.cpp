#include "utils.h"

namespace utils {

    ExpectedType<finances::accounts::models::Amount, utils::libpqxx::ParseNumericError>
    qstring_to_amount(const QString& input, const finances::accounts::models::Ccy& ccy) {
        // TODO: Different Ccy might have different parse formats
        static const dec::decimal_format SPANISH_DECIMAL_FORMAT{','}; // FIXME: Get this from Qt locale
        return finances::accounts::models::Amount::parse(input.toStdString(), SPANISH_DECIMAL_FORMAT);
    }

    QDate date_to_qdate(const utils::libpqxx::Date& date) {
        return QDate{int(date.year()), static_cast<int>(unsigned(date.month())),
                     static_cast<int>(unsigned(date.day()))};
    }

} // namespace utils

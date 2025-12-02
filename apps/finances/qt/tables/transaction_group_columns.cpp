#include "transaction_group_columns.h"

#include "apps/finances/qt/utils/utils.h"

using namespace finances::accounts::models;

namespace utils::qt::models {

    template <>
    QVariant
    DataDispatcher<finances::accounts::models::TransactionGroup, TransactionGroupColumns, Qt::DisplayRole>::data(
        const finances::accounts::models::TransactionGroup& tg, TransactionGroupColumns column) {

        QVariant result = QVariant();

        switch (column) {
        case TransactionGroupColumns::ID:
            result.setValue(tg.id);
            break;
        case TransactionGroupColumns::NAME:
            result = QString::fromStdString(tg.name);
            break;
        case TransactionGroupColumns::DESCRIPTION:
            if (tg.description) {
                result = QString::fromStdString(tg.description.value());
            }
            break;
        case TransactionGroupColumns::START:
            result = utils::date_to_qdate(tg.start).toString("yyyy-MM-dd");
            break;
        case TransactionGroupColumns::END:
            if (tg.end) {
                result = utils::date_to_qdate(tg.end.value()).toString("yyyy-MM-dd");
            }
            break;
        }

        return result;
    }

} // namespace utils::qt::models

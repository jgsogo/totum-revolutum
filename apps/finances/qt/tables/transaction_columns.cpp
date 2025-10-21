#include "transaction_columns.h"

using namespace finances::accounts::models;

namespace utils::qt::models {

    template <>
    QVariant DataDispatcher<Transaction, TransactionColumns, Qt::DisplayRole>::data(const Transaction& transaction,
                                                                                    TransactionColumns column) {
        QVariant result = QVariant();

        switch (column) {
        case TransactionColumns::ID:
            result.setValue(transaction.id);
            break;
        case TransactionColumns::NAME:
            result = QString::fromStdString(transaction.name);
            break;
        case TransactionColumns::DESCRIPTION:
            if (transaction.description) {
                result = QString::fromStdString(transaction.description.value());
            }
            break;
        case TransactionColumns::GROUP_ID:
            if (transaction.group) {
                result.setValue(transaction.group.value().first);
            }
            break;
        case TransactionColumns::GROUP_NAME:
            if (transaction.group) {
                result.setValue(transaction.group.value().second);
            }
            break;
        }

        return result;
    }

} // namespace utils::qt::models

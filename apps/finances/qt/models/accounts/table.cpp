#include "table.h"

AccountsTable::AccountsTable(utils::libpqxx::ConnectionPool& pool, QObject* parent)
    : utils::qt::models::TableModel<AccountModel, AccountColumns>{pool, parent} {}

namespace utils::qt::models {

    template <>
    QVariant
    DataDispatcher<AccountModel, AccountColumns, Qt::DisplayRole>::data(const TableModel<AccountModel, AccountColumns>&,
                                                                        const AccountModel&, AccountColumns) {
        return QVariant{QString::fromStdString("lol")};
    }

} // namespace utils::qt::models

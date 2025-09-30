#include "account_detail.h"

AccountDetailWidget::AccountDetailWidget(utils::libpqxx::ConnectionPool& pool_,
                                         const finances::accounts::models::Account& account_,
                                         const MovementTypeTableModel* movtype_model_, QWidget* parent)
    : QWidget(parent), pool{pool_}, account{account_}, movtype_model{movtype_model_} {}

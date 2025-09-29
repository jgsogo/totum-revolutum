#include "account_detail.h"

AccountDetailWidget::AccountDetailWidget(utils::libpqxx::ConnectionPool& pool_,
                                         const finances::accounts::models::Account& account_, QWidget* parent)
    : QWidget(parent), pool{pool_}, account{account_} {}

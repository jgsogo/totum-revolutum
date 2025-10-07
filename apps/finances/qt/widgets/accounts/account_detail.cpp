#include "account_detail.h"

AccountDetailWidget::AccountDetailWidget(utils::libpqxx::ConnectionPool& pool_,
                                         const finances::accounts::models::Account& account_,
                                         const MovementTypeTableModel* movtype_model_, QWidget* parent)
    : QWidget(parent), pool{pool_}, account{account_}, movtype_model{movtype_model_} {}

void AccountDetailWidget::on_new_snapshot(finances::accounts::models::Id account_id) {
    SPDLOG_DEBUG("AccountDetailWidget::on_new_snapshot(account_id={})", account_id);
    assert(account_id == account.id);
    emit snapshot_added(account_id);
}

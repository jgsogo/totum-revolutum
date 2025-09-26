#include "account_detail.h"

#include <QLabel>
#include <QVBoxLayout>

AccountDetailWidget::AccountDetailWidget(const finances::accounts::models::Account& account_,
                                         AccountSnapshotsModel* snapshots_model, AccountMovementsModel* movements_model,
                                         QWidget* parent)
    : QWidget(parent), account{account_} {

    QLabel* name = new QLabel(QString::fromStdString(account.name));

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(name);

    this->setLayout(mainLayout);
}

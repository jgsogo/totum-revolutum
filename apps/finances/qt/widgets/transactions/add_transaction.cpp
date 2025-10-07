#include "add_transaction.h"

#include <QDialogButtonBox>
#include <QLabel>
#include <QVBoxLayout>
#include <spdlog/spdlog.h>

void AddTransactionWidget::add_transaction_clicked() {
    SPDLOG_DEBUG("AddTransactionWidget::add_transaction_clicked()");
}
void AddTransactionWidget::add_movement(finances::accounts::models::MovementDirection) {
    SPDLOG_DEBUG("AddTransactionWidget::add_movement()");
}

void AddTransactionWidget::remove_movement(/* we need some kind of ID here */) {
    SPDLOG_DEBUG("AddTransactionWidget::remove_movement()");
}

AddTransactionWidget::AddTransactionWidget(utils::libpqxx::ConnectionPool& pool_, QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool_} {

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &AddTransactionWidget::add_transaction_clicked);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QVBoxLayout* mainLayout = new QVBoxLayout;
    QString title(tr("Add transaction"));
    mainLayout->addWidget(new QLabel(title));
    // mainLayout->addLayout(formLayout);
    mainLayout->addWidget(buttonBox);

    this->setLayout(mainLayout);
    this->setWindowTitle(title);
}

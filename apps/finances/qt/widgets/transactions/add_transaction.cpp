#include "add_transaction.h"

#include <spdlog/spdlog.h>

#include <QDialogButtonBox>
#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QStackedLayout>
#include <QVBoxLayout>

#include "movements/add_movement.h"
// #include "movements/add_movement_dividend.h"
// #include "movements/add_movement_non_numerable.h"
// #include "movements/add_movement_numerable.h"

void AddTransactionWidget::add_transaction_clicked() {
    SPDLOG_DEBUG("AddTransactionWidget::add_transaction_clicked()");
}
void AddTransactionWidget::add_movement(finances::accounts::models::MovementDirection direction) {
    SPDLOG_DEBUG("AddTransactionWidget::add_movement(direction={})", direction);

    AddMovementWidget* mov = new AddMovementWidget;

    switch (direction) {
    case finances::accounts::models::MovementDirection::IN: {
        movs_in_layout->addWidget(mov);
        // movs_in.push_back(stackedLayout);
    } break;
    case finances::accounts::models::MovementDirection::OUT: {
        movs_out_layout->addWidget(mov);
        // movs_out.push_back(stackedLayout);
    } break;
    }
}

void AddTransactionWidget::remove_movement(/* we need some kind of ID here */) {
    SPDLOG_DEBUG("AddTransactionWidget::remove_movement()");
}

AddTransactionWidget::AddTransactionWidget(utils::libpqxx::ConnectionPool& pool_, QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool_} {

    // Transaction details

    // Movements
    movs_in_layout = new QVBoxLayout;
    QPushButton* add_movs_in = new QPushButton(tr("+ IN"), this);
    connect(add_movs_in, &QPushButton::clicked,
            [this]() { this->add_movement(finances::accounts::models::MovementDirection::IN); });
    movs_in_layout->addWidget(add_movs_in);
    this->add_movement(finances::accounts::models::MovementDirection::IN);

    movs_out_layout = new QVBoxLayout;
    QPushButton* add_movs_out = new QPushButton(tr("+ OUT"), this);
    connect(add_movs_out, &QPushButton::clicked,
            [this]() { this->add_movement(finances::accounts::models::MovementDirection::OUT); });
    movs_out_layout->addWidget(add_movs_out);
    this->add_movement(finances::accounts::models::MovementDirection::OUT);

    //
    QHBoxLayout* all_movs_layout = new QHBoxLayout();
    all_movs_layout->addLayout(movs_in_layout);
    all_movs_layout->addLayout(movs_out_layout);

    // Layout main
    QVBoxLayout* mainLayout = new QVBoxLayout;
    QString title(tr("Add transaction"));
    mainLayout->addWidget(new QLabel(title));

    mainLayout->addLayout(all_movs_layout);

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &AddTransactionWidget::add_transaction_clicked);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);
    mainLayout->addWidget(buttonBox);

    this->setLayout(mainLayout);
    this->setWindowTitle(title);
}

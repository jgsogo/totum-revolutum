#include "add_transaction.h"

#include <QDialogButtonBox>
#include <QHeaderView>
#include <QLabel>
#include <QPushButton>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QVBoxLayout>

#include "add_movement.h"

AddTransactionWidget::AddTransactionWidget(utils::libpqxx::ConnectionPool& pool,
                                           AccountsTableModel<AccountColumns>& accounts_,
                                           MovementTypesTableModel<HierarchyTreeColumns>& movtypes_, QWidget* parent,
                                           Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool}, money_in{finances::accounts::models::EUR},
      money_out{finances::accounts::models::EUR} {

    // Models
    // - the list of movements associated to this account
    movements = new MovementsForTransactionTableModel<MovementColumns>(transaction, pool, this);

    QPushButton* add_movement = new QPushButton(tr("Add movement"), this);
    {
        AddMovementWidget* popup_add_movement = new AddMovementWidget(pool, accounts_, movtypes_, this);
        popup_add_movement->setModal(true);
        popup_add_movement->setSizeGripEnabled(true);

        connect(add_movement, &QPushButton::clicked, [popup_add_movement]() {
            popup_add_movement->clear();
            popup_add_movement->open();
        });

        connect(popup_add_movement, &AddMovementWidget::new_movement, this, &AddTransactionWidget::on_new_movement);
    }

    // Components
    QTableView* table_view = new QTableView(this);
    {
        QSortFilterProxyModel* sort_filter = new QSortFilterProxyModel(this);
        sort_filter->setSourceModel(movements);
        sort_filter->sort(magic_enum::enum_integer(MovementColumns::DATE_VALUE), Qt::DescendingOrder);

        table_view->setModel(sort_filter);
        table_view->setSortingEnabled(false);
        table_view->hideColumn(magic_enum::enum_integer(MovementColumns::ID));
        table_view->verticalHeader()->hide();
        table_view->horizontalHeader()->setSectionResizeMode(QHeaderView::ResizeToContents);

        table_view->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        table_view->resizeColumnsToContents();
        table_view->resizeRowsToContents();
    }

    money_in_label = new QLabel(tr("IN: %1").arg(static_cast<std::string>(money_in)));
    money_out_label = new QLabel(tr("IN: %1").arg(static_cast<std::string>(money_in)));

    buttonBox = new QDialogButtonBox(QDialogButtonBox::Save | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &QDialog::accept);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QHBoxLayout* bottom_line = new QHBoxLayout;
    bottom_line->addWidget(money_in_label);
    bottom_line->addWidget(money_out_label);
    bottom_line->addWidget(buttonBox);

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(new QLabel(tr("Add new transaction")));
    mainLayout->addWidget(add_movement);
    mainLayout->addWidget(table_view);
    mainLayout->addLayout(bottom_line);

    this->setLayout(mainLayout);
}

void AddTransactionWidget::on_new_movement(MovementModel movement) {
    SPDLOG_DEBUG("AddTransactionWidget::on_new_movement(movement)");

    const auto& plain_movement = movement.as_movement();

    switch (plain_movement.direction) {
    case finances::accounts::models::MovementDirection::IN:
        money_in += plain_movement.amount_in_local_ccy();
        break;
    case finances::accounts::models::MovementDirection::OUT:
        money_out += plain_movement.amount_in_local_ccy();
        break;
    }

    money_in_label->setText(tr("IN: %1").arg(static_cast<std::string>(money_in)));
    money_out_label->setText(tr("OUT: %1").arg(static_cast<std::string>(money_out)));

    QPushButton* accept_button = buttonBox->button(QDialogButtonBox::Save);
    if (money_in == money_out) {
        accept_button->setEnabled(true);
    } else {
        accept_button->setEnabled(false);
    }

    movements->insert(std::move(movement));
}

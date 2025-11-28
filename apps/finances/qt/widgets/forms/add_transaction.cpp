#include "add_transaction.h"

#include <QDialogButtonBox>
#include <QFormLayout>
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

    transaction_title = new QLineEdit(this);
    transaction_description = new QTextEdit(this);
    transaction_description->setAcceptRichText(false);

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
        table_view->hideColumn(magic_enum::enum_integer(MovementColumns::TRANSACTION_ID));
        table_view->hideColumn(magic_enum::enum_integer(MovementColumns::ACCOUNT_ID));
        table_view->hideColumn(magic_enum::enum_integer(MovementColumns::TRANSACTION));
        table_view->verticalHeader()->hide();
        table_view->horizontalHeader()->setSectionResizeMode(QHeaderView::ResizeToContents);

        table_view->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        table_view->resizeColumnsToContents();
        table_view->resizeRowsToContents();
    }

    money_in_label = new QLabel(tr("IN: %1").arg(static_cast<std::string>(money_in)));
    money_out_label = new QLabel(tr("OUT: %1").arg(static_cast<std::string>(money_out)));

    buttonBox = new QDialogButtonBox(QDialogButtonBox::Save | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &AddTransactionWidget::add_transaction_clicked);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QHBoxLayout* bottom_line = new QHBoxLayout;
    bottom_line->addWidget(money_in_label);
    bottom_line->addWidget(money_out_label);
    bottom_line->addWidget(buttonBox);

    QFormLayout* formLayout = new QFormLayout;
    formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
    formLayout->addRow(tr("&Title:"), transaction_title);
    formLayout->addRow(tr("&Description:"), transaction_description);

    // Layout
    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(new QLabel(tr("Add new transaction")));
    mainLayout->addLayout(formLayout);
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
    if (this->is_valid()) {
        accept_button->setEnabled(true);
    } else {
        accept_button->setEnabled(false);
    }

    movements->insert(std::move(movement));
}

bool AddTransactionWidget::is_valid() const {
    // Transaction needs a title
    return ((transaction_title->hasAcceptableInput()) && (money_in == money_out) && (movements->rowCount() != 0));
}

void AddTransactionWidget::add_transaction_clicked() {
    SPDLOG_DEBUG("AddTransactionWidget::add_transaction_clicked()");

    if (!this->is_valid()) {
        SPDLOG_WARN("User was able to click the 'Save' button, but the transaction is invalid. We cannot add it.");
        return;
    }

    // Transaction
    QString description = transaction_description->toPlainText();
    finances::accounts::models::Transaction new_transaction{
        .id = {std::monostate{}}, // No id, it's not in the database yet!
        .name = this->transaction_title->text().toStdString(),
        .description = description.isEmpty() ? std::nullopt : std::optional<std::string>{description.toStdString()},
        .group = std::nullopt,
    };

    // Movements
    const auto& all_movements = movements->all();

    // Now I need to insert all the movements and the transaction using a single DB transaction, or rollback everything.
    bool success = pool.with_conn<bool>([&new_transaction, &all_movements](pqxx::connection& conn) {
        SPDLOG_DEBUG("Insert the new transaction and all the movements using the same DB transaction");
        pqxx::work tx(conn);
        try {
            auto transaction_id = utils::db::ModelManager<finances::accounts::models::Transaction>::_create(
                tx, std::move(new_transaction));
            for (auto mov : all_movements) {
                mov.as_mut_movement().transaction.first = transaction_id;
                utils::db::ModelManager<MovementModel>::_create(tx, mov);
            }
            tx.commit();
            return true;
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to create new transaction: {}", e.what());
            return false;
        }
    });

    // Notify to all the accounts involved, that there are new movements and they need to update their data.
    if (success) {
        for (const auto& movement : all_movements) {
            emit new_movement(movement.as_movement().account.first);
        }
        this->accept();
    } else {
        // TODO: Notify the user about the error
    }
}

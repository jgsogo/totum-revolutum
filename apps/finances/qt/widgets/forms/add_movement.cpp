#include "add_movement.h"

#include <spdlog/spdlog.h>

#include <QDialogButtonBox>
#include <QFormLayout>
#include <QGridLayout>

AddMovementWidget::AddMovementWidget(utils::libpqxx::ConnectionPool& pool,
                                     AccountsTableModel<AccountColumns>& accounts_, QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool} {

    account_combo =
        new utils::qt::widgets::ComboBoxWithSearch{accounts_, AccountColumns::ID, AccountColumns::NAME, parent};

    movtype = new QComboBox;

    mov_date = new QCalendarWidget;

    mov_amount = new widgets::forms::MovementStackedForm;

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &QDialog::accept);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QFormLayout* formLayout = new QFormLayout(this);
    formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
    formLayout->addRow(tr("&Account:"), account_combo);

    formLayout->addRow(tr("Movement &type:"), movtype);
    formLayout->addRow(tr("Movement &date:"), mov_date);
    formLayout->addRow(tr("&Amount:"), mov_amount);

    QGridLayout* layout = new QGridLayout;
    layout->addLayout(formLayout, 0, 0);
    layout->addWidget(buttonBox, 1, 0);
    this->setLayout(layout);

    account_combo->setFocus();
}

void AddMovementWidget::account_changed() { SPDLOG_DEBUG("AddMovementWidget::account_changed()"); }

void AddMovementWidget::movtype_changed() { SPDLOG_DEBUG("AddMovementWidget::movtype_changed()"); }

void AddMovementWidget::show_mov_date() { SPDLOG_DEBUG("AddMovementWidget::show_mov_date()"); }

void AddMovementWidget::hide_mov_date(QDate date) { SPDLOG_DEBUG("AddMovementWidget::hide_mov_date()"); }

void AddMovementWidget::mov_date_changed(QDate date) { SPDLOG_DEBUG("AddMovementWidget::mov_date_changed()"); }

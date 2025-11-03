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

    movtype_combo = new QComboBox;

    mov_date = new QCalendarWidget;

    mov_amount = new widgets::forms::MovementStackedForm;

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &QDialog::accept);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QFormLayout* formLayout = new QFormLayout;
    formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
    formLayout->addRow(tr("&Account:"), account_combo);

    formLayout->addRow(tr("Movement &type:"), movtype_combo);
    formLayout->addRow(tr("Movement &date:"), mov_date);
    // formLayout->addRow(tr("&Amount:"), mov_amount);

    QGridLayout* layout = new QGridLayout;
    layout->addLayout(formLayout, 0, 0);
    layout->addWidget(mov_amount, 1, 0);
    layout->addWidget(buttonBox, 2, 0);
    this->setLayout(layout);

    connect(account_combo, &utils::qt::widgets::_ComboBoxWithSearch::activated, [this, &accounts_](utils::db::Id id) {
        auto account_expected = accounts_.get(id);
        if (!account_expected) {
            SPDLOG_ERROR("Cannot get an account with id '{}'", id);
            // TODO: Communicate error to the user
            return;
        }
        this->account_changed(account_expected.value());
    });

    account_combo->setFocus();
}

void AddMovementWidget::clear(bool keep_date) {
    this->account_combo->clearEditText();
    this->movtype_combo->clearEditText();
    this->mov_amount->clear();
    if (!keep_date) {
        this->mov_date->setSelectedDate(QDate::currentDate());
    }
}

void AddMovementWidget::account_changed(const AccountModel& account) {
    SPDLOG_DEBUG("AddMovementWidget::account_changed(account.name={})", account.account.name);
    mov_amount->set_ccy(account.account.ccy);
    if (account.account.is_numerable) {
        mov_amount->set_movement_numerable();
    } else {
        mov_amount->set_movement_non_numerable();
    }
}

void AddMovementWidget::movtype_changed() { SPDLOG_DEBUG("AddMovementWidget::movtype_changed()"); }

void AddMovementWidget::show_mov_date() { SPDLOG_DEBUG("AddMovementWidget::show_mov_date()"); }

void AddMovementWidget::hide_mov_date(QDate date) { SPDLOG_DEBUG("AddMovementWidget::hide_mov_date()"); }

void AddMovementWidget::mov_date_changed(QDate date) { SPDLOG_DEBUG("AddMovementWidget::mov_date_changed()"); }

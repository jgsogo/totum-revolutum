#include "add_movement.h"

#include <spdlog/spdlog.h>

#include <QVBoxLayout>

AddMovementWidget::AddMovementWidget(/*AccountTableModel* accounts,*/ QWidget* parent) : QWidget(parent) {

    mov_amount = new widgets::forms::MovementStackedForm;

    account = new QComboBox;
    // QCompleter *mycompletear = new QCompleter(this);
    // mycompletear->setCaseSensitivity(Qt::CaseInsensitive);
    // mycompletear->setModel(proxyModel);
    // mycompletear->setCompletionColumn(1);
    // mycompletear->setCompletionMode(QCompleter::UnfilteredPopupCompletion);
    // ui->comp_comb->setCompleter(mycompletear);
    // account->setModel(accounts);
    // account->setModelColumn(1);

    movtype = new QComboBox;

    mov_date = new QCalendarWidget;

    QVBoxLayout* layout = new QVBoxLayout;
    layout->addWidget(account);
    layout->addWidget(movtype);
    layout->addWidget(mov_date);
    layout->addWidget(mov_amount);

    this->setLayout(layout);
}

void AddMovementWidget::account_changed() { SPDLOG_DEBUG("AddMovementWidget::account_changed()"); }

void AddMovementWidget::movtype_changed() { SPDLOG_DEBUG("AddMovementWidget::movtype_changed()"); }

void AddMovementWidget::show_mov_date() { SPDLOG_DEBUG("AddMovementWidget::show_mov_date()"); }

void AddMovementWidget::hide_mov_date(QDate date) { SPDLOG_DEBUG("AddMovementWidget::hide_mov_date()"); }

void AddMovementWidget::mov_date_changed(QDate date) { SPDLOG_DEBUG("AddMovementWidget::mov_date_changed()"); }

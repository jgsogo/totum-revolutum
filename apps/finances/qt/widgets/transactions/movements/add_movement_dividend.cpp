#include "add_movement_dividend.h"

AddMovementDividendWidget::AddMovementDividendWidget(QWidget* parent) : AddMovementWidget(parent) {}

void AddMovementDividendWidget::ex_dividend_date_changed() {
    SPDLOG_DEBUG("AddMovementDividendWidget::ex_dividend_date_changed()");
}

void AddMovementDividendWidget::unit_value_changed() {
    SPDLOG_DEBUG("AddMovementDividendWidget::unit_value_changed()");
}

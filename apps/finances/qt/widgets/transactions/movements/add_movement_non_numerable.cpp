#include "add_movement_non_numerable.h"

AddMovementNonNumerableWidget::AddMovementNonNumerableWidget(QWidget* parent) : AddMovementWidget(parent) {}

void AddMovementNonNumerableWidget::amount_changed() {
    SPDLOG_DEBUG("AddMovementNonNumerableWidget::amount_changed()");
}

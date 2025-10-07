#include "add_movement_numerable.h"

AddMovementNumerableWidget::AddMovementNumerableWidget(QWidget* parent) : AddMovementWidget(parent) {}

void AddMovementNumerableWidget::quantity_changed() { SPDLOG_DEBUG("AddMovementNumerableWidget::quantity_changed()"); }

void AddMovementNumerableWidget::unit_value_changed() {
    SPDLOG_DEBUG("AddMovementNumerableWidget::unit_value_changed()");
}

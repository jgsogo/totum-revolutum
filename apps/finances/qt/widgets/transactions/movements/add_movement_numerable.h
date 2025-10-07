#pragma once

#include "add_movement.h"

class AddMovementNumerableWidget : public AddMovementWidget {
    Q_OBJECT

  public:
    explicit AddMovementNumerableWidget(QWidget* parent = nullptr);

  private slots:
    // When the quantity changes, the amount will change
    void quantity_changed();

    // When unit value changes, the amount will change
    void unit_value_changed();

  protected:
    QLineEdit* quantity;
    QLineEdit* unit_value;
};

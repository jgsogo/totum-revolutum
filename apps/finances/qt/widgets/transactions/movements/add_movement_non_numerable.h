#pragma once

#include "add_movement.h"

class AddMovementNonNumerableWidget : public AddMovementWidget {
    Q_OBJECT

  public:
    explicit AddMovementNonNumerableWidget(QWidget* parent = nullptr);

  private slots:
    // When unit value changes, the amount will change
    void amount_changed();

  protected:
    QLineEdit* amount;
};

#pragma once

#include "add_movement.h"

class AddMovementDividendWidget : public AddMovementWidget {
    Q_OBJECT

  public:
    explicit AddMovementDividendWidget(QWidget* parent = nullptr);

  private slots:
    // When the ex dividend date changes, we might need to retrieve snapshots, and quantity and amount will change
    void ex_dividend_date_changed();

    // When unit value changes, the amount will change
    void unit_value_changed();

  protected:
    QLineEdit* unit_value;
    QCalendarWidget* ex_dividend_date;
};

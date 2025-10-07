#pragma once

#include <QCalendarWidget>
#include <QComboBox>
#include <QWidget>

#include "libraries/finances/accounts/cpp/models/types/amount.h"

class AddMovementWidget : public QWidget {
    Q_OBJECT

  public:
    explicit AddMovementWidget(QWidget* parent = nullptr);
    virtual ~AddMovementWidget() = default;

  private slots:
    // The selected account has changed. The fields in the widget might change.
    void account_changed();

    // The selected movement type has changed. Fields in the widget might change.
    void movtype_changed();

  public slots:
    // Show the calendar to choose the date
    void show_mov_date();

    // Hide the calendar. Pass the date that will be used for this movement
    void hide_mov_date(QDate date);

    // When the calendar is hidden, any change to mov dates should be communicated from outside
    void mov_date_changed(QDate date);

  signals:
    // When the amount changes, someone outside might be interested on knowing it
    void amount_changed(finances::accounts::models::Amount);

  protected:
    QComboBox* account;
    QComboBox* movtype;
    QCalendarWidget* mov_date;
};

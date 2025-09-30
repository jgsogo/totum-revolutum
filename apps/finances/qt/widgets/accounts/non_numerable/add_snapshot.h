#pragma once

#include <QCalendarWidget>
#include <QDialog>
#include <QLineEdit>

#include "libraries/finances/accounts/cpp/models/account.h"

#include "apps/finances/qt/metatypes/snapshot_non_numerable.h"

class AddSnapshotNonNumerableWidget : public QDialog {
    Q_OBJECT

  private slots:
    void add_snapshot_clicked();

  signals:
    void new_snapshot(SnapshotNonNumerable);

  public:
    AddSnapshotNonNumerableWidget(const finances::accounts::models::Account& account, QWidget* parent = nullptr,
                                  Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    const finances::accounts::models::Account& account;
    QCalendarWidget* calendar;
    QLineEdit* amount;
};

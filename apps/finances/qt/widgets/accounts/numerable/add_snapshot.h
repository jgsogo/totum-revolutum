#pragma once

#include <QCalendarWidget>
#include <QDialog>
#include <QLineEdit>

#include "apps/finances/qt/metatypes/snapshot_numerable.h"

class AddSnapshotNumerableWidget : public QDialog {
    Q_OBJECT

  private slots:
    void add_snapshot_clicked();

  signals:
    void new_snapshot(SnapshotNumerable);

  public:
    AddSnapshotNumerableWidget(QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    QCalendarWidget* calendar;
    QLineEdit* quantity;
    QLineEdit* unit_value;
};

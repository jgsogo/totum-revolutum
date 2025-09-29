#pragma once

#include <QCalendarWidget>
#include <QDialog>
#include <QLineEdit>

#include "apps/finances/qt/metatypes/snapshot_non_numerable.h"

class AddSnapshotNonNumerableWidget : public QDialog {
    Q_OBJECT

  private slots:
    void add_snapshot_clicked();

  signals:
    void new_snapshot(SnapshotNonNumerable);

  public:
    AddSnapshotNonNumerableWidget(QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    QCalendarWidget* calendar;
    QLineEdit* amount;
};

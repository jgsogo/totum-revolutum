#pragma once

#include <QCalendarWidget>
#include <QDialog>
#include <QLineEdit>

#include "libraries/finances/accounts/cpp/models/types/amount.h"
#include "libraries/utils/cpp/libpqxx/datatypes/date.h"

class Snapshot {
  public:
    Snapshot() = default;
    ~Snapshot() = default;
    Snapshot(const Snapshot&) = default;
    Snapshot& operator=(const Snapshot&) = default;

    Snapshot(const QDate& date, const QString& amount);

    QDate date() const;
    QStringView amount() const;

  private:
    QDate date_;
    QString amount_;
};
Q_DECLARE_METATYPE(Snapshot);

class AddSnapshotWidget : public QDialog {
    Q_OBJECT

  private slots:
    void add_snapshot();

  signals:
    void new_snapshot(Snapshot);

  public:
    AddSnapshotWidget(QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    QCalendarWidget* calendar;
    QLineEdit* amount;
};

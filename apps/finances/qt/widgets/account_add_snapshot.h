#pragma once

#include <QCalendarWidget>
#include <QDialog>
#include <QLineEdit>

#include "libraries/finances/accounts/cpp/models/types/amount.h"
#include "libraries/utils/cpp/libpqxx/datatypes/date.h"

template <std::size_t MaxDigits, std::size_t DecimalPlaces> struct Snapshot {
    // dec::decimal maximum number of digits is 18 (uses 64 bit integer under the hood)
    static_assert(MaxDigits <= 18, "MaxDigits cannot excceed 18");

  public:
  public:
    Snapshot() = default;
    ~Snapshot() = default;
    Snapshot(const Snapshot&) = default;
    Snapshot& operator=(const Snapshot&) = default;

    Snapshot(const QDate& date, const QString& amount) : date_{date} {
        static const dec::decimal_format SPANISH_DECIMAL_FORMAT{','};
        amount_ = dec::fromString<dec::decimal<DecimalPlaces>>(amount.toStdString(), SPANISH_DECIMAL_FORMAT);
    }

    QDate date() const { return date_; }
    dec::decimal<DecimalPlaces> amount() const { return amount_; };

  private:
    QDate date_;
    dec::decimal<DecimalPlaces> amount_;
};

using Snapshot2Decs = Snapshot<14, 4>;
Q_DECLARE_METATYPE(Snapshot2Decs);

class AddSnapshotWidget : public QDialog {
    Q_OBJECT

  private slots:
    void add_snapshot();

  signals:
    void new_snapshot(Snapshot2Decs);

  public:
    AddSnapshotWidget(QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    QCalendarWidget* calendar;
    QLineEdit* amount;
};

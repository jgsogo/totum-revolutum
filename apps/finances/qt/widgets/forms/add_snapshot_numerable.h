#pragma once

#include <QCalendarWidget>
#include <QDialog>

#include "libraries/finances/accounts/cpp/models/account.h"

#include "apps/finances/qt/widgets/forms/amounts/money_amount.h"

class AddSnapshotNumerableWidget : public QDialog {
    Q_OBJECT

  private slots:
    void add_snapshot_clicked();
    void on_inputs_changed();

  signals:
    void new_snapshot(utils::db::Id account_id);

  public:
    AddSnapshotNumerableWidget(utils::libpqxx::ConnectionPool& pool, const finances::accounts::models::Account& account,
                               QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

    ExpectedType<finances::accounts::models::Money> getMoneyAmount() const;

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
    QCalendarWidget* calendar;
    QLineEdit* quantity;
    MoneyAmountEdit* unit_value;

    QLabel* amount_label;
};

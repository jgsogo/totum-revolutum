#pragma once

#include <QCalendarWidget>
#include <QDialog>
#include <QLineEdit>

#include "libraries/finances/accounts/cpp/models/account.h"

class AddSnapshotNumerableWidget : public QDialog {
    Q_OBJECT

  private slots:
    void add_snapshot_clicked();

  signals:
    void new_snapshot(utils::db::Id account_id);

  public:
    AddSnapshotNumerableWidget(utils::libpqxx::ConnectionPool& pool, const finances::accounts::models::Account& account,
                               QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
    QCalendarWidget* calendar;
    QLineEdit* quantity;
    QLineEdit* unit_value;
};

#pragma once

#include <QCalendarWidget>
#include <QDialog>
#include <QLineEdit>

#include "libraries/finances/accounts/cpp/models/account.h"

class AddSnapshotNonNumerableWidget : public QDialog {
    Q_OBJECT

  private slots:
    void add_snapshot_clicked();

  signals:
    void new_snapshot(finances::accounts::models::Id account_id);

  public:
    AddSnapshotNonNumerableWidget(utils::libpqxx::ConnectionPool& pool,
                                  const finances::accounts::models::Account& account, QWidget* parent = nullptr,
                                  Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
    QCalendarWidget* calendar;
    QLineEdit* amount;
};

#pragma once

#include <QObject>
#include <pqxx/pqxx>

#include "libraries/finances/accounts/cpp/models/types/id.h"

/*
A class to manage all the DB notifications
*/
class Notificator : public QObject {
    Q_OBJECT
  public:
    // This object will maintain a connection opened with the server
    Notificator(pqxx::connection&& conn, std::chrono::milliseconds notification_loop, QObject* parent = nullptr);

  private slots:
    void check_notifications();

  public slots:
    void notify_all_accounts();
    void notify_account(finances::accounts::models::Id account_id);

  signals:
    void account_changed(finances::accounts::models::Id account_id);

  private:
    pqxx::connection conn;
};

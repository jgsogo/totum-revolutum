#pragma once

#include <QObject>
#include <pqxx/pqxx>

#include "libraries/utils/cpp/libpqxx/orm/id.h"

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
    void notify_account(utils::db::Id account_id);

  signals:
    void account_changed(utils::db::Id account_id);

  private:
    pqxx::connection conn;
};

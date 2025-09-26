#include "notificator.h"

#include <QTimer>

#include "libraries/utils/cpp/libpqxx/notify.h"

constexpr static std::string_view ACCOUNT_CHANNEL = "account";

Notificator::Notificator(pqxx::connection&& _conn, std::chrono::milliseconds notification_loop, QObject* parent)
    : QObject(parent), conn{std::move(_conn)} {
    SPDLOG_DEBUG("Listen notification on channel '{}'", ACCOUNT_CHANNEL);
    conn.listen(ACCOUNT_CHANNEL, [&](pqxx::notification n) {
        SPDLOG_TRACE("DB notification received on channel '{}' with payload '{}'", ACCOUNT_CHANNEL, n.payload);
        if (!n.payload.empty()) {
            uint64_t account_id_u = std::stoull(std::string(n.payload));
            finances::accounts::models::Id account_id{account_id_u};
            emit account_changed(account_id);
        } else {
            SPDLOG_WARN("All accounts changed notification is not implemented!");
        }
    });

    SPDLOG_DEBUG("Configure a time to call 'Notificator::check_notifications' every {} milliseconds",
                 notification_loop.count());
    QTimer* timer = new QTimer(this);
    connect(timer, &QTimer::timeout, this, &Notificator::check_notifications);
    timer->start(notification_loop);
}

void Notificator::check_notifications() {
    SPDLOG_TRACE("Notificator::check_notifications");
    conn.get_notifs();
}

void Notificator::notify_all_accounts() { utils::libpqxx::notify(conn, ACCOUNT_CHANNEL); }

void Notificator::notify_account(finances::accounts::models::Id account_id) {
    utils::libpqxx::notify(conn, ACCOUNT_CHANNEL, std::to_string(account_id));
}

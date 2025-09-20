#include "notify.h"
#include <spdlog/spdlog.h>

namespace utils::db {

    namespace {

        int _send_notify(pqxx::connection& conn, std::string_view notification) {
            SPDLOG_DEBUG("Send DB notification: {}", notification);

            pqxx::work tx(conn);
            try {
                tx.exec(notification).no_rows();
                tx.commit();
                return 0;
            } catch (std::exception const& e) {
                SPDLOG_ERROR("Error sending NOTIFY: {}", e.what());
                return -1;
            }
        }
    } // namespace

    int notify(pqxx::connection& conn, std::string_view channel, std::string_view payload) {
        auto query = std::format("NOTIFY {}, '{}'", channel, payload);
        return _send_notify(conn, query);
    }

    int notify(pqxx::connection& conn, std::string_view channel) {
        auto query = std::format("NOTIFY {}", channel);
        return _send_notify(conn, query);
    }
} // namespace utils::db

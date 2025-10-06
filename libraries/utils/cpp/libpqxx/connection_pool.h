#pragma once

#include <condition_variable>
#include <memory>
#include <mutex>
#include <pqxx/pqxx>
#include <queue>
#include <tl/expected.hpp>

namespace utils::libpqxx {

    class ConnectionPool {
      public:
        ConnectionPool(const std::string& conninfo, std::size_t pool_size);

        static tl::expected<ConnectionPool, std::string> from_env(std::string_view prefix, std::size_t pool_size);
        static ConnectionPool from(std::string_view sql_database, std::string_view sql_user,
                                   std::string_view sql_password, std::string_view sql_host, std::string_view sql_port,
                                   std::size_t pool_size);

        std::shared_ptr<pqxx::connection> acquire();
        void release(std::shared_ptr<pqxx::connection> conn);

        template <typename R> R with_conn(std::function<R(pqxx::connection& conn)> work) {
            auto conn = this->acquire();
            auto r = work(*conn);
            this->release(conn);
            return r;
        }

        std::queue<std::shared_ptr<pqxx::connection>> drain();

      private:
        std::queue<std::shared_ptr<pqxx::connection>> pool;
        std::mutex mutex;
        std::condition_variable cond;
    };

    template <> void ConnectionPool::with_conn<void>(std::function<void(pqxx::connection& conn)> work);

} // namespace utils::libpqxx

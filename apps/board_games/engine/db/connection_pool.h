#pragma once

#include <condition_variable>
#include <memory>
#include <mutex>
#include <pqxx/pqxx>
#include <queue>

namespace db {
    class ConnectionPool {
      public:
        ConnectionPool(const std::string& conninfo, std::size_t pool_size);

        static ConnectionPool from_env(const std::string& prefix, std::size_t pool_size);

        std::shared_ptr<pqxx::connection> acquire();
        void release(std::shared_ptr<pqxx::connection> conn);

        template <typename R> R with_conn(std::function<R(pqxx::connection& conn)> work) {
            auto conn = this->acquire();
            auto r = work(*conn);
            this->release(conn);
            return r;
        }

      private:
        std::queue<std::shared_ptr<pqxx::connection>> pool;
        std::mutex mutex;
        std::condition_variable cond;
    };

    template <> void ConnectionPool::with_conn<void>(std::function<void(pqxx::connection& conn)> work);

} // namespace db

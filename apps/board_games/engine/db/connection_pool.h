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

  private:
    std::queue<std::shared_ptr<pqxx::connection>> pool;
    std::mutex mutex;
    std::condition_variable cond;
};
} // namespace db

#include "connection_pool.h"

#include <iostream>
#include <spdlog/spdlog.h>

using namespace utils::libpqxx;

namespace {
    [[nodiscard]]
    std::string connection_string(std::string_view sql_database, std::string_view sql_user,
                                  std::string_view sql_password, std::string_view sql_host, std::string_view sql_port) {
        const std::string connection_str = std::format("dbname={} user={} password={} host={} port={}", sql_database,
                                                       sql_user, sql_password, sql_host, sql_port);
        return connection_str;
    }
} // namespace

ConnectionPool::ConnectionPool(const std::string& conninfo, std::size_t pool_size) {
    for (std::size_t i = 0; i < pool_size; ++i) {
        pool.emplace(std::make_shared<pqxx::connection>(conninfo));
    }
}

utils::ExpectedType<ConnectionPool, MissingEnvvar> ConnectionPool::from_env(std::string_view prefix,
                                                                            std::size_t pool_size) {
    // FIXME: Return some kind of error, not just a std::string
    auto sql_database = std::getenv(std::format("{}SQL_DATABASE", prefix).c_str());
    if (sql_database == nullptr) {
        return tl::unexpected{MissingEnvvar{std::format("{}SQL_DATABASE", prefix)}};
    }

    auto sql_user = std::getenv(std::format("{}SQL_USER", prefix).c_str());
    if (sql_user == nullptr) {
        return tl::unexpected{MissingEnvvar{std::format("{}SQL_USER", prefix)}};
    }

    auto sql_password = std::getenv(std::format("{}SQL_PASSWORD", prefix).c_str());
    if (sql_password == nullptr) {
        return tl::unexpected{MissingEnvvar{std::format("{}SQL_PASSWORD", prefix)}};
    }

    auto sql_host = std::getenv(std::format("{}SQL_HOST", prefix).c_str());
    if (sql_host == nullptr) {
        return tl::unexpected{MissingEnvvar{std::format("{}SQL_HOST", prefix)}};
    }

    auto sql_port = std::getenv(std::format("{}SQL_PORT", prefix).c_str());
    if (sql_port == nullptr) {
        return tl::unexpected{MissingEnvvar{std::format("{}SQL_PORT", prefix)}};
    }

    const std::string connstr = connection_string(sql_database, sql_user, sql_password, sql_host, sql_port);
    SPDLOG_DEBUG("Connection string: {}", connstr);
    return utils::ExpectedType<ConnectionPool, MissingEnvvar>{tl::in_place, connstr, pool_size};
}

ConnectionPool ConnectionPool::from(std::string_view sql_database, std::string_view sql_user,
                                    std::string_view sql_password, std::string_view sql_host, std::string_view sql_port,
                                    std::size_t pool_size) {
    const std::string connection_str = connection_string(sql_database, sql_user, sql_password, sql_host, sql_port);
    SPDLOG_DEBUG("Connection string: {}", connection_str);
    return ConnectionPool{connection_str, pool_size};
}

std::shared_ptr<pqxx::connection> ConnectionPool::acquire() {
    std::unique_lock<std::mutex> lock(mutex);
    cond.wait(lock, [this]() { return !pool.empty(); });

    auto conn = pool.front();
    pool.pop();
    return conn;
}

void ConnectionPool::release(std::shared_ptr<pqxx::connection> conn) {
    std::lock_guard<std::mutex> lock(mutex);
    pool.push(conn);
    cond.notify_one();
}

template <> void ConnectionPool::with_conn<void>(std::function<void(pqxx::connection& conn)> work) {
    auto conn = this->acquire();
    work(*conn);
    this->release(conn);
}

std::queue<std::shared_ptr<pqxx::connection>> ConnectionPool::drain() {
    std::lock_guard<std::mutex> lock(mutex);

    // Clearing the queue by swapping with an empty queue
    std::queue<std::shared_ptr<pqxx::connection>> empty;
    std::swap(pool, empty);
    return empty;
}

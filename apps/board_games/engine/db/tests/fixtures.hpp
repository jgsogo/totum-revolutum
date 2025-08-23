#pragma once

#include "apps/board_games/engine/db/connection_pool.h"

constexpr static std::string_view PREFIX = "BOARD_GAMES_ENGINE_";

// Provides a `db::ConnectionPool` to a temporal database that is a clone of the
// 'BOARD_GAMES_ENGINE_' one.
class UniqueDBConnectionPool {
  public:
    UniqueDBConnectionPool() : pool(cloned_db()) {}

    ~UniqueDBConnectionPool() {
        // Get name of the cloned database and close all connections to it
        const std::string cloned_dbname =
            pool.with_conn<std::string>([](pqxx::connection& conn) { return conn.dbname(); });
        std::ignore = pool.drain();

        // Remove the cloned DB. I need to use a connection to a different database
        db::ConnectionPool::from_env(PREFIX, 1).with_conn<void>(
            [&cloned_dbname](pqxx::connection& conn) { // FIXME: Use a single connection instead of a pool
                pqxx::nontransaction tx{conn};
                tx.exec(std::format("DROP DATABASE {}", cloned_dbname));
            });
    }

    static db::ConnectionPool cloned_db() {
        // Ensure only one connection is created to the database (otherwise the CREATE DATABASE TEMPLATE fails)
        static std::mutex mtx;
        std::unique_lock<std::mutex> lock(mtx);

        // Clone the original DB and return the new prefix
        auto original_db = db::ConnectionPool::from_env(PREFIX, 1); // FIXME: Use a single connection instead of a pool
        std::string new_db = original_db.with_conn<std::string>([](pqxx::connection& conn) {
            static int clone_idx = 0; // TODO: Ensure uniqueness. Use UUID
            std::string tmp_database = std::format("{}{}_db", PREFIX, clone_idx++);
            std::transform(tmp_database.begin(), tmp_database.end(), tmp_database.begin(),
                           [](unsigned char c) { return std::tolower(c); });

            pqxx::nontransaction tx{conn};
            const char* original_sql_database = std::getenv(std::format("{}SQL_DATABASE", PREFIX).c_str());
            const char* original_sql_user = std::getenv(std::format("{}SQL_USER", PREFIX).c_str());
            tx.exec(std::format("CREATE DATABASE {} TEMPLATE {} OWNER {};", tmp_database, original_sql_database,
                                original_sql_user))
                .no_rows();
            tx.exec(std::format("GRANT CONNECT, CREATE, TEMP ON DATABASE {} TO {};", tmp_database, original_sql_user))
                .no_rows();

            return tmp_database;
        });

        const char* sql_user = std::getenv(std::format("{}SQL_USER", PREFIX).c_str());
        const char* sql_password = std::getenv(std::format("{}SQL_PASSWORD", PREFIX).c_str());
        const char* sql_host = std::getenv(std::format("{}SQL_HOST", PREFIX).c_str());
        const char* sql_port = std::getenv(std::format("{}SQL_PORT", PREFIX).c_str());
        return db::ConnectionPool::from(new_db, sql_user, sql_password, sql_host, sql_port, 4);
    }

  public:
    mutable db::ConnectionPool pool;
};

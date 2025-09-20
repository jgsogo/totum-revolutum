#pragma once

#include <stduuid/uuid.h>

#include "libraries/utils/cpp/db/connection_pool.h"
#include "libraries/utils/cpp/string_literal.hpp"

// Provides a `db::ConnectionPool` to a temporal database that clones the "default" one
template <utils::StringLiteral PREFIX> class UniqueDBConnectionPoolWithPrefix {
  public:
    UniqueDBConnectionPoolWithPrefix() : pool(cloned_db()) {}

    ~UniqueDBConnectionPoolWithPrefix() {
        // Get name of the cloned database and close all connections to it
        const std::string cloned_dbname =
            pool.with_conn<std::string>([](pqxx::connection& conn) { return conn.dbname(); });
        std::ignore = pool.drain();

        // Remove the cloned DB. I need to use a connection to a different database
        db::ConnectionPool::from_env(PREFIX, 1).template with_conn<void>(
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
        std::string new_db = original_db.template with_conn<std::string>([](pqxx::connection& conn) {
            // Use UUID so there are no collisions even across multiple processes, this way it would
            // be possible to use the same PostgreSQL instance for all the tests in the repo.
            std::string id = uuids::to_string(uuids::uuid_system_generator{}());

            // Database names need to be lowercase and some chars are forbidden: '-'
            std::string tmp_database = std::format("{}p_{}_db", PREFIX, id);
            std::transform(tmp_database.begin(), tmp_database.end(), tmp_database.begin(),
                           [](unsigned char c) { return c == '-' ? '_' : std::tolower(c); });

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

using UniqueDBConnectionPool = UniqueDBConnectionPoolWithPrefix<"">;

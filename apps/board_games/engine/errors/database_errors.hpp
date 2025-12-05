#pragma once

#include "libraries/utils/cpp/expected_type.hpp"

namespace errors {

    //! Errors raised as a result of some interaction with a database
    using DatabaseError = utils::errors::BaseError<class DatabaseErrorTag>;

    struct InsertError : public DatabaseError {
        InsertError(std::string_view table_name, const std::string& query, const std::string& reason)
            : DatabaseError(std::format("Error inserting to table '{}': {} ('{}')", table_name, reason, query)) {};
    };

    struct SelectError : public DatabaseError {
        SelectError(std::string_view table_name, const std::string& query, const std::string& reason)
            : DatabaseError(std::format("Error selecting from table '{}': {} ('{}')", table_name, reason, query)) {};
    };

    struct UpdateError : public DatabaseError {
        UpdateError(std::string_view table_name, const std::string& query, const std::string& reason)
            : DatabaseError(std::format("Error updating row in table '{}': {} ('{}')", table_name, reason, query)) {};
    };

    struct DBNotificationError : public DatabaseError {
        DBNotificationError(const std::string& reason)
            : DatabaseError{std::format("Error sending DB notification: {}", reason)} {};
    };
} // namespace errors

template <> struct fmt::formatter<errors::DatabaseError> : fmt::formatter<std::string> {
    auto format(errors::DatabaseError p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "DatabaseError: {}", p.msg);
    }
};

template <> struct std::formatter<errors::DatabaseError> : std::formatter<std::string> {
    auto format(const errors::DatabaseError& p, std::format_context& ctx) const {
        return std::formatter<std::string>::format(std::format("DatabaseError: {}", p.msg), ctx);
    }
};

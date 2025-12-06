#pragma once

#include "libraries/utils/cpp/expected_type/expected_type.hpp"

namespace errors {

    //! Errors raised as a result of some interaction with a database
    using DatabaseError = utils::errors::BaseError<"DatabaseError">;

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

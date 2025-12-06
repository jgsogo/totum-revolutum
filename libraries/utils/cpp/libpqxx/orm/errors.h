
#pragma once

#include "libraries/utils/cpp/expected_type/errors.hpp"

namespace utils::db {

    using DatabaseError = errors::BaseError<"DatabaseError">;
    using ErrorNotFound = errors::BaseError<"NotFoundError">;
    using ErrorMultipleFound = errors::BaseError<"ErrorMultipleFound">;
    using ErrorInvalidInput = errors::BaseError<"ErrorInvalidInput">;
} // namespace utils::db

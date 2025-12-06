#pragma once

#include "libraries/utils/cpp/expected_type/expected_type.hpp"

namespace errors {

    //! Errors raised because some invariant was not fulfilled.
    //!
    //! It should be possible to avoid these errors by modifying the application logic
    using LogicalError = utils::errors::BaseError<"LogicalError">;

} // namespace errors

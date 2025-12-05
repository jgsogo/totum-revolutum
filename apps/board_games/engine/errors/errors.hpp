#pragma once

#include "database_errors.hpp"
#include "logical_errors.hpp"
#include "runtime_errors.hpp"

template <typename T>
using Expected = utils::ExpectedType<T, errors::DatabaseError, errors::LogicalError, errors::RuntimeError>;

#pragma once

#include <stdexcept>

namespace finances::accounts::models {

    // FIXME: Use strong types instead, see https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjhxti2/,
    //        and we can write some widening-variant like https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjrfl5z/
    //        and move this to a reusable taget built on top of std::expected
    enum class Error {
        InsertError,
        SelectError,
        DBError,
        NotifcationFailed,
        NotFound,
        GameEngineError,
        GameDecodeError,
        GameIsFinished,
        GameActionFailed,
        NotImplemented,
        GameEncodeError
    };

} // namespace finances::accounts::models

namespace finances::accounts::models::error {

    struct CcyMismatch : public std::runtime_error {
        CcyMismatch() : std::runtime_error("Cannot operate on Money instances with different currencies") {};
    };

    struct FXRateInvalid : public std::runtime_error {
        FXRateInvalid(const std::string& fx) : std::runtime_error(std::format("Invalid FX: {}", fx)) {};
    };

} // namespace finances::accounts::models::error

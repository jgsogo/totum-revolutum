#pragma once

#include "libraries/utils/cpp/expected_type.hpp"
#include "libraries/utils/cpp/type_name.hpp"

namespace errors {

    //! Errors raised because some input data is invalid.
    using RuntimeError = utils::errors::BaseError<class RuntimeErrorTag>;

    template <typename T> struct StringToTypeError : public RuntimeError {
        StringToTypeError(std::string_view input)
            : RuntimeError(std::format("Cannot convert input '{}' to type '{}'", input, utils::type_name<T>())) {};
    };

    template <typename T, typename TProto> struct ConvertToProtoError : public RuntimeError {
        ConvertToProtoError(const std::string& reason)
            : RuntimeError(std::format("Failed to convert instance of '{}' to proto '{}': {}", utils::type_name<T>(),
                                       utils::type_name<TProto>(), reason)) {};
    };

    template <typename T, typename TProto> struct ConvertFromProtoError : public RuntimeError {
        ConvertFromProtoError(const std::string& reason)
            : RuntimeError(std::format("Failed to convert instance of proto '{}' to instance of class '{}': {}",
                                       utils::type_name<TProto>(), utils::type_name<T>(), reason)) {};
    };

    struct GameEngineError : public RuntimeError {
        GameEngineError(const std::string& msg) : RuntimeError(msg) {};
    };

    struct InvalidData : public GameEngineError {
        InvalidData(const std::string& msg) : GameEngineError(msg) {};
    };

    struct InvalidAction : public GameEngineError {
        InvalidAction(const std::string& msg) : GameEngineError(std::format("Action is invalid: {}", msg)) {};
    };

} // namespace errors

template <> struct fmt::formatter<errors::RuntimeError> : fmt::formatter<std::string> {
    auto format(errors::RuntimeError p, format_context& ctx) const -> decltype(ctx.out()) {
        return fmt::format_to(ctx.out(), "RuntimeError: {}", p.msg);
    }
};

template <> struct std::formatter<errors::RuntimeError> : std::formatter<std::string> {
    auto format(const errors::RuntimeError& p, std::format_context& ctx) const {
        return std::formatter<std::string>::format(std::format("RuntimeError: {}", p.msg), ctx);
    }
};

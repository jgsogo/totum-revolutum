#pragma once

#include "apps/board_games/engine/errors/errors.hpp"

namespace data {

    namespace _detail {
        template <typename T> class Payload {
          public:
            Payload() = delete;
            Payload(const Payload&) = default;            // TODO: Remove this copy ctors
            Payload& operator=(const Payload&) = default; // TODO: Remove this copy ctors

            explicit Payload(Payload&& payload) noexcept = default;
            Payload& operator=(Payload&&) noexcept = default;

            explicit Payload(std::vector<std::byte>&& payload) : _payload{std::move(payload)} {};

            operator pqxx::bytes_view() const { return pqxx::bytes_view{_payload.begin(), _payload.end()}; }

            template <typename TProto> Expected<TProto> into_proto() const {
                TProto proto;
                if (!proto.ParseFromArray(_payload.data(), _payload.size())) {
                    return tl::unexpected{errors::ConvertToProtoError<T, TProto>{"ParseFromArray failed"}};
                }
                return {proto};
            }

            template <typename TProto> static Expected<Payload> from_proto(TProto&& proto) {
                std::vector<std::byte> payload{proto.ByteSizeLong()};
                if (!proto.SerializeToArray(payload.data(), payload.size())) {
                    return tl::unexpected{errors::ConvertFromProtoError<T, TProto>{"SerializeToArray failed"}};
                }
                Payload obj{std::move(payload)};
                return Expected<Payload>{std::move(obj)};
            }

          protected:
            std::vector<std::byte> _payload;
        };
    } // namespace _detail

    using GamePayload = _detail::Payload<class GamePayloadTypeTag>;
    using ActionPayload = _detail::Payload<class ActionPayloadTypeTag>;
    using EventPayload = _detail::Payload<class EventPayloadTypeTag>;

} // namespace data

// // Custom datatype for libpqxx: https://libpqxx.readthedocs.io/stable/datatypes.html#autotoc_md10,
// // most of the implementation taken from https://gist.github.com/tomlankhorst/5c41127a3f4fe3e6b1b4cb114ec7e3be
namespace pqxx {
    template <> inline std::string const type_name<data::GamePayload>{"GamePayload"};
    template <> inline std::string const type_name<data::ActionPayload>{"ActionPayload"};
    template <> inline std::string const type_name<data::EventPayload>{"EventPayload"};

    template <typename T> struct nullness<data::_detail::Payload<T>> : no_null<data::_detail::Payload<T>> {};

    template <typename T> struct string_traits<data::_detail::Payload<T>> {
        static data::_detail::Payload<T> from_string(std::string_view text) {
            auto bytes = string_traits<pqxx::bytes>::from_string(text);
            std::vector<std::byte> as_vector{
                bytes.begin(), bytes.end()}; // FIXME: There is a copy here, use std::basic_string<std::byte> everywhere
            return data::_detail::Payload<T>{std::move(as_vector)};
        }

        static zview to_buf(char* begin, char* end, const data::_detail::Payload<T>& value) {
            return string_traits<pqxx::bytes_view>::to_buf(begin, end, value);
        }

        static char* into_buf(char* begin, char* end, const data::_detail::Payload<T>& value) {
            return string_traits<pqxx::bytes_view>::into_buf(begin, end, value);
        }

        static std::size_t size_buffer(const data::_detail::Payload<T>& value) noexcept {
            return string_traits<pqxx::bytes_view>::size_buffer(value);
        }
    };
} // namespace pqxx

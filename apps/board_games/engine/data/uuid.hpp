#pragma once

#include <pqxx/pqxx>
#include <string>

namespace data {

    namespace _detail {
        enum UUIDType {
            Room,
            Participant,
        };

        template <UUIDType T> class UUID {
          public:
            explicit UUID(std::string&& value) : value{std::move(value)} {}
            UUID() : value{NULL_UUID} {}

            operator std::string_view() const { return value; }

            [[nodiscard]] static UUID<T> null() { return UUID<T>{}; }
            bool is_null() const { return value == NULL_UUID; }

          private:
            constexpr static std::string_view NULL_UUID = "00000000-0000-0000-0000-000000000000";
            std::string value; // Use boost::uuid? overkill?
        };
    } // namespace _detail
} // namespace data

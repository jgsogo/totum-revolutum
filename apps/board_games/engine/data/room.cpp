#include "room.h"
#include <spdlog/spdlog.h>

namespace data {

    static constexpr std::string_view ROOMS_TABLE = "board_games_core_room";

    std::expected<void, Error> insert_new_room(pqxx::connection& conn, RoomUUID uuid, std::string_view name) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Insert new room with uuid '{}'", uuid);
            tx.exec("INSERT INTO $1 (id, name) VALUES ($2, $3);", pqxx::params{ROOMS_TABLE, uuid, name}).no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_DEBUG("Failed to insert new room: {}", e.what());
            return std::unexpected(Error::InsertError);
        }
    }

    std::expected<std::vector<RoomUUID>, Error> get_playing_rooms(pqxx::connection& conn) {
        try {
            pqxx::work tx(conn);
            std::vector<RoomUUID> ret;
            for (auto [id] : tx.query<RoomUUID>("SELECT id FROM $1;", pqxx::params{ROOMS_TABLE})) {
                ret.emplace_back(id);
            }
            return ret;
        } catch (const std::exception& e) {
            SPDLOG_DEBUG("Failed to retrieve list of rooms: {}", e.what());
            return std::unexpected(Error::SelectError);
        }
    }

} // namespace data

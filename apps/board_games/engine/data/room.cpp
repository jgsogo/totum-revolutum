#include "room.h"
#include <spdlog/spdlog.h>

namespace data {

    static constexpr std::string_view ROOMS_TABLE = "board_games_core_room";
    static constexpr std::string_view GAMES_TABLE = "board_games_core_game";

    tl::expected<void, Error> insert_new_room(pqxx::connection& conn, RoomUUID uuid, std::string_view name) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Insert new room with uuid '{}'", static_cast<std::string_view>(uuid));
            tx.exec(std::format("INSERT INTO {} (id, name) VALUES ($1, $2);", ROOMS_TABLE), pqxx::params{uuid, name})
                .no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert new room: {}", e.what());
            return tl::unexpected(Error::InsertError);
        }
    }

    tl::expected<std::vector<RoomUUID>, Error> get_playing_rooms(pqxx::connection& conn) {
        try {
            pqxx::work tx(conn);
            std::vector<RoomUUID> ret;
            SPDLOG_DEBUG("Get list of playing room");
            for (auto [id] : tx.query<RoomUUID>(std::format("SELECT id FROM {};", ROOMS_TABLE))) {
                ret.emplace_back(id);
            }
            return ret;
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to retrieve list of rooms: {}", e.what());
            return tl::unexpected(Error::SelectError);
        }
    }

    tl::expected<void, Error> start_game(pqxx::connection& conn, RoomUUID uuid, GameType game) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Remove game from room '{}'", static_cast<std::string_view>(uuid));
            tx.exec(std::format("INSERT INTO {} (room_id, game_type) VALUES ($1, $2);", GAMES_TABLE),
                    pqxx::params{uuid, game})
                .no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to insert game: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }

    tl::expected<void, Error> remove_game(pqxx::connection& conn, RoomUUID uuid) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Remove game from room '{}'", static_cast<std::string_view>(uuid));
            tx.exec(std::format("DELETE FROM {} WHERE room_id = $1;", GAMES_TABLE), pqxx::params{uuid}).no_rows();
            tx.commit();
            return {};
        } catch (const std::exception& e) {
            SPDLOG_ERROR("Failed to remove game: {}", e.what());
            return tl::unexpected(Error::DBError);
        }
    }
} // namespace data

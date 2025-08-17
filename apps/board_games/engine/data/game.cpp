#include "game.h"
#include <spdlog/spdlog.h>

namespace data {

    static constexpr std::string_view GAMES_TABLE = "board_games_core_game";

    tl::expected<void, Error> start_game(pqxx::connection& conn, RoomUUID uuid, GameType game) {
        try {
            pqxx::work tx(conn);
            SPDLOG_DEBUG("Remove game from room '{}'", static_cast<std::string_view>(uuid));
            tx.exec(std::format("INSERT INTO {} (room_id, game_type_id, created_at, updated_at, state, state_data) "
                                "VALUES ($1, $2, NOW(), NOW(), 'waiting', '');",
                                GAMES_TABLE),
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

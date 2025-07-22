#include "get_playing_room_ids.h"

std::vector<std::string> db::get_playing_room_ids(pqxx::connection& conn) {
    pqxx::work tx(conn);
    std::vector<std::string> res;
    for (auto [id] : tx.query<std::string>("SELECT id FROM board_games_core_room;")) {
        res.emplace_back(id);
    }
    return res;
}

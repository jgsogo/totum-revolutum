#include "get_playing_room_ids.h"

#include "notify.h"
#include <iostream>

std::vector<std::string> db::get_playing_room_ids(pqxx::connection& conn) {
    // TODO: Remove, this is just sending a notification
    ::db::notify(conn, "game_update", "payload");

    pqxx::work tx2(conn);
    std::vector<std::string> res;
    for (auto [id] : tx2.query<std::string>("SELECT id FROM board_games_core_room;")) {
        res.emplace_back(id);
    }
    return res;
}

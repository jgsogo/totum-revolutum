#include "get_playing_room_ids.h"

#include <iostream>

std::vector<std::string> db::get_playing_room_ids(pqxx::connection& conn) {
    pqxx::work tx(conn);

    // TODO: Remove. Just a place to send a notify call
    try {
        tx.exec("NOTIFY game_update, 'payload'").no_rows();
        tx.commit();
    } catch (std::exception const& e) {
        std::cerr << e.what() << std::endl;
    }

    pqxx::work tx2(conn);
    std::vector<std::string> res;
    for (auto [id] : tx2.query<std::string>("SELECT id FROM board_games_core_room;")) {
        res.emplace_back(id);
    }
    return res;
}

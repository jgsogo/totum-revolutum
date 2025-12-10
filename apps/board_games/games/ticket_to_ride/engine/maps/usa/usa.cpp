#include "usa.h"

#include <spdlog/spdlog.h>

namespace board_games::ticket_to_ride {

    void populate_usa_map(MapData& map) {
        SPDLOG_ERROR("Here I need to read the text proto file");
        map.set_name("USA");
    }
} // namespace board_games::ticket_to_ride

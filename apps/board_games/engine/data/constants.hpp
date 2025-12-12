#pragma once

#include <string_view>

namespace data {

    // Tables
    static constexpr std::string_view GAMES_TABLE = "board_games_core_game";
    static constexpr std::string_view GAME_TYPE_TABLE = "board_games_core_gametype";
    static constexpr std::string_view EVENT_LOG_TABLE = "board_games_core_event";
    static constexpr std::string_view GAME_ACTION_TABLE = "board_games_core_action";
    static constexpr std::string_view PARTICIPANT_TABLE = "board_games_core_participant";
    static constexpr std::string_view SNAPSHOT_TABLE = "board_games_core_snapshot";
    static constexpr std::string_view ROOMS_TABLE = "board_games_core_room";

    // Notification channels
    static constexpr std::string_view NOTIFICATION_CHANNEL_ROOM = "room_update";

} // namespace data

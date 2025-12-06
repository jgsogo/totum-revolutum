#pragma once

#include <expected>
#include <pqxx/pqxx>
#include <vector>

#include "apps/board_games/engine/errors/errors.hpp"

#include "models/game.hpp"
#include "models/participant.hpp"
#include "models/participant_role.hpp"
#include "models/uuid_participant.hpp"
#include "models/uuid_room.hpp"

namespace data {

    Expected<std::vector<RoomUUID>> get_playing_rooms(pqxx::connection& conn);

    Expected<std::optional<Game>> find_game(pqxx::connection& conn, RoomUUID room);

    Expected<void> insert_new_room(pqxx::connection& conn, RoomUUID room, std::string_view name);

    Expected<std::optional<Participant>> find_participant(pqxx::connection& conn, RoomUUID room,
                                                          ParticipantUUID participant);

    Expected<Participant> add_participant(pqxx::connection& conn, RoomUUID room, ParticipantUUID participant,
                                          ParticipantRole role, std::optional<uint32_t> player_number = std::nullopt);

    Expected<void> notify_room_update(pqxx::connection& conn, RoomUUID room);

} // namespace data

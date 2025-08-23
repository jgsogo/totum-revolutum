#pragma once

#include <expected>
#include <pqxx/pqxx>
#include <vector>

#include "tl/expected.hpp"

#include "errors.h"
#include "models/game.hpp"
#include "models/participant.hpp"
#include "models/participant_role.hpp"
#include "models/uuid_participant.hpp"
#include "models/uuid_room.hpp"

namespace data {

    tl::expected<std::vector<RoomUUID>, Error> get_playing_rooms(pqxx::connection& conn);

    tl::expected<std::optional<Game>, Error> find_game(pqxx::connection& conn, RoomUUID room);

    tl::expected<void, Error> insert_new_room(pqxx::connection& conn, RoomUUID room, std::string_view name);

    tl::expected<std::optional<Participant>, Error> find_participant(pqxx::connection& conn, RoomUUID room,
                                                                     ParticipantUUID participant);

    tl::expected<Participant, Error> add_participant(pqxx::connection& conn, RoomUUID room, ParticipantUUID participant,
                                                     ParticipantRole role);

    tl::expected<void, Error> notify_room_update(pqxx::connection& conn, RoomUUID room);

} // namespace data

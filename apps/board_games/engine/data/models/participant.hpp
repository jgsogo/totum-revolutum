#pragma once

#include "participant_role.hpp"
#include "uuid_participant.hpp"
#include "uuid_room.hpp"

namespace data {
    struct Participant {
        RoomUUID room;
        ParticipantUUID uuid;
        ParticipantRole role;
        std::uint32_t player_number;
    };
} // namespace data

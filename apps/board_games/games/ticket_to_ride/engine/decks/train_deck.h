#pragma once

#include "apps/board_games/games/ticket_to_ride/models/decks.pb.h"

namespace board_games::ticket_to_ride {

    void populate_train_deck(TrainDeck& train_deck, int seed);

}

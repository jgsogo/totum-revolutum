#pragma once

#include <filesystem>

#include "libraries/utils/cpp/expected_type/expected_type.hpp"

#include "apps/board_games/games/ticket_to_ride/maps/map.pb.h"

// TODO: Here it's applied to the 'MapData', but this is generic functionality to load protos

namespace board_games::ticket_to_ride {

    using ErrorLoadingFile = utils::errors::BaseError<"ErrorLoadingFile">;
    using ErrorTextProtoParse = utils::errors::BaseError<"ErrorTextProtoParse">;
    using ErrorBinaryProtoParse = utils::errors::BaseError<"ErrorBinaryProtoParse">;

    utils::ExpectedType<MapData, ErrorLoadingFile, ErrorTextProtoParse>
    load_map_data(const std::filesystem::path& path);

    utils::ExpectedType<MapData, ErrorLoadingFile, ErrorBinaryProtoParse>
    load_map_data_binary(const std::filesystem::path& path);

} // namespace board_games::ticket_to_ride

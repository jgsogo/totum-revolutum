#pragma once

#include <memory>
#include <string_view>
#include <unordered_map>

#include "apps/board_games/engine/data/models/game_type.hpp"

#include "game_plugin.hpp"

namespace engine {

    struct _GameTypeHasher {
        std::size_t operator()(const data::GameType& k) const { return std::hash<std::string_view>()(k); }
    };

    using GamePluginsMap = std::unordered_map<data::GameType, std::unique_ptr<GamePluginBase>, _GameTypeHasher>;
} // namespace engine

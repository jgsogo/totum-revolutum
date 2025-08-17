#pragma once

#include "apps/board_games/engine/db/connection_pool.h"

class DBConnectionPool {
  public:
    DBConnectionPool() : pool(db::ConnectionPool::from_env("BOARD_GAMES_ENGINE_", 4)) {}

  public:
    mutable db::ConnectionPool pool;
};

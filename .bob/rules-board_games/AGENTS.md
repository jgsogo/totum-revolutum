# Board Games App Mode Rules

This file contains app-specific patterns and constraints for the board_games application.

## Application Architecture

- **Multi-game platform**: Supports multiple board games (Tic-Tac-Toe, Ticket to Ride) with extensible architecture
- **C++ game engine**: Core game logic in C++ with protobuf-based state serialization
- **TypeScript webapp**: SvelteKit frontend for web-based gameplay
- **Real-time updates**: Uses PostgreSQL LISTEN/NOTIFY for room-based event streaming
- **Protobuf communication**: Engine and webapp communicate via protobuf messages

## Database Schema (PostgreSQL)

See `apps/board_games/db.md` for complete schema documentation. Key tables:

- **game_types**: Supported game types (chess, checkers, etc.) identified by slug
- **games**: Game instances with protobuf-encoded state in `state` column (bytea)
- **rooms**: Virtual spaces where players gather (status: waiting/playing/ended)
- **players**: Participants in rooms (ephemeral/anonymous names)
- **moves**: Move history with protobuf-encoded move data
- **event**: System events and state transitions (jsonb payload)

**Design principle**: Rooms and games are separate for semantic clarity and future extensibility (e.g., multiple games per room, tournaments)

## Development Workflow

- **Dev environment**: `bazel run //apps/board_games/engine:dev` - starts Postgres, runs migrations, populates test data, launches engine
- **Engine only**: `bazel run //apps/board_games/engine:engine`
- **CLI tool**: `bazel run //apps/board_games/engine/cli` - command-line interface for testing
- **Webapp dev**: Standard SvelteKit development in `apps/board_games/webapp/`
- **Database migrations**: Django-based migrations in `apps/board_games/database/`

## Protobuf Patterns (Critical)

- **State serialization**: Game state stored as protobuf bytea in `games.state` column
- **Move encoding**: Each move stored as protobuf bytea in `moves.move_data` column
- **Multi-language**: Same .proto files generate code for C++ (engine) and TypeScript (webapp)
- **Update workflow**: After modifying protos, run `just update-npm` to copy generated TS files to workspace
- **Engine protocol**: Located in `apps/board_games/engine/protocol/`

## Build Patterns (App-Specific)

- **C++23 required**: All C++ code uses C++23 standard
- **Postgres tests**: Use `with_postgres_test()` wrapper - automatically manages Docker container lifecycle
- **Test data**: Migrations are `test_compile_data`, populated data is runtime
- **Engine binary**: Single binary supports multiple game types via plugin-like architecture

## Game Engine Architecture

- **Game type registration**: Each game (Tic-Tac-Toe, Ticket to Ride) registers with engine
- **State management**: Authoritative state in database, engine validates and applies moves
- **Move validation**: Engine validates moves before applying to state
- **Replay capability**: Move history allows game replay for auditing/visualization

## Real-Time Communication

- **LISTEN/NOTIFY**: Web backend subscribes to `room_<uuid>` channels for updates
- **WebSocket integration**: Frontend connects via WebSocket to receive room events
- **Event types**: Game state changes, player joins/leaves, move notifications
- **Room status**: Transitions between waiting → playing → ended

## Testing Patterns

- **Integration tests**: Located in `apps/board_games/engine/data/tests/`
- **Test database**: `_populate_test_database` target creates test fixtures
- **Game logic tests**: Test game state transitions, move validation, win conditions
- **Room tests**: Test room lifecycle and player management

## Database Management

- **Django migrations**: Schema managed by Django in `apps/board_games/database/`
- **Migration command**: `bazel run //apps/board_games/database:app-migrate -- migrate`
- **Test population**: `bazel run //apps/board_games/database:_populate_test_database`
- **Connection pool**: Engine uses libpqxx connection pool for database access

## Webapp Patterns

- **SvelteKit framework**: Located in `apps/board_games/webapp/`
- **Server-side rendering**: Uses SvelteKit's SSR capabilities
- **WebSocket client**: Connects to engine for real-time updates
- **Protobuf parsing**: TypeScript code parses protobuf messages from engine
- **Room subscription**: Clients subscribe to specific room channels

## Common Gotchas

- **Protobuf sync**: Always run `just update-npm` after modifying .proto files
- **State serialization**: Game state must be fully serializable to protobuf
- **UUID primary keys**: All tables use UUIDs for distributed-friendly design
- **Timestamps in UTC**: All timestamps use UTC with `NOW()` default
- **Room-game separation**: Don't conflate rooms (spaces) with games (activities)
- **LISTEN/NOTIFY channels**: Channel names follow pattern `room_<uuid>`

## Future Enhancements (Documented)

- Game templates table for pre-configured setups
- Teams/roles for complex games
- Materialized views for analytics
- Tournament support
- Multiple games per room

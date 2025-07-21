# 🗃️ Board Games Application – Database Schema Documentation

This document provides an exhaustive reference for the PostgreSQL schema used in the *board-games* application.

---

## 📑 Tables Overview

### 1. `game_types`
Stores information about the types of games supported by the system (e.g. chess, checkers).

| Column        | Type        | Description                                |
|---------------|-------------|--------------------------------------------|
| `id`          | `uuid`      | Primary key                                |
| `slug`        | `text`      | Unique identifier used by the engine/UI    |
| `name`        | `text`      | Human-readable name of the game            |
| `description` | `text`      | Optional description of the game           |
| `created_at`  | `timestamp` | Time of creation                           |

**Notes:**
Each game type defines its rules externally in the engine. This table allows multi-game support in the same database.

---

### 2. `games`
Represents a specific game instance (e.g. a chess match between two players).

| Column        | Type        | Description                                      |
|---------------|-------------|--------------------------------------------------|
| `id`          | `uuid`      | Primary key                                      |
| `game_type_id`| `uuid`      | FK to `game_types(id)`                          |
| `room_id`     | `uuid`      | FK to `rooms(id)`, links game to a room         |
| `state`       | `bytea`     | Binary protobuf-encoded game state              |
| `created_at`  | `timestamp` | Game creation time                              |
| `updated_at`  | `timestamp` | Last update time                                |
| `ended_at`    | `timestamp` | Time the game ended (nullable)                  |

**Notes:**
The `state` column stores the authoritative game state in serialized protobuf format. This allows the engine to restore a game at any time.

---

### 3. `rooms`
Represents a room where players gather and play a game.

| Column        | Type        | Description                           |
|---------------|-------------|---------------------------------------|
| `id`          | `uuid`      | Primary key                           |
| `name`        | `text`      | Human-readable name                   |
| `status`      | `text`      | Status: `waiting`, `playing`, `ended`|
| `created_at`  | `timestamp` | Time of creation                      |
| `updated_at`  | `timestamp` | Last updated                          |

**Notes:**
A room may only have one game active at a time. The web UI subscribes to room events via `LISTEN room_<id>`.

---

### 4. `players`
Represents a player participating in a room.

| Column        | Type        | Description                                |
|---------------|-------------|--------------------------------------------|
| `id`          | `uuid`      | Primary key                                |
| `room_id`     | `uuid`      | FK to `rooms(id)`                          |
| `name`        | `text`      | Player display name                        |
| `joined_at`   | `timestamp` | Time the player joined the room            |

**Notes:**
Player authentication is not managed. Player names are assumed to be ephemeral or anonymous initially.

---

### 5. `moves`
Stores the history of moves performed by players in a game.

| Column        | Type        | Description                                   |
|---------------|-------------|-----------------------------------------------|
| `id`          | `uuid`      | Primary key                                   |
| `game_id`     | `uuid`      | FK to `games(id)`                             |
| `player_id`   | `uuid`      | FK to `players(id)`                           |
| `move_data`   | `bytea`     | Protobuf-encoded move                         |
| `created_at`  | `timestamp` | Time the move was performed                   |

**Notes:**
Each move is stored in protobuf format and can be replayed for auditing or visualization.

---

### 6. `event_log`
Logs notable events, such as game state transitions or system actions.

| Column        | Type        | Description                            |
|---------------|-------------|----------------------------------------|
| `id`          | `uuid`      | Primary key                            |
| `game_id`     | `uuid`      | FK to `games(id)`                      |
| `event_type`  | `text`      | Describes the event type               |
| `payload`     | `jsonb`     | Additional context as JSON             |
| `created_at`  | `timestamp` | When the event occurred                |

**Notes:**
Used for debugging, analytics, and UI enhancements.

---

## 📌 Additional Design Notes

- All timestamps use UTC and default to `NOW()` at insertion.
- All primary keys are UUIDs for easy distribution.
- Protobuf is the serialization format for game data and moves.
- Web backend uses `LISTEN/NOTIFY` on room channels (e.g. `room_<uuid>`) to push updates to clients.

---

## 🧠 Future Enhancements

- Add a `game_templates` table for pre-configured rules or setups
- Add `teams` or `roles` for more complex games
- Store parsed state in a materialized view for analytics

---


## ✅ Why keep rooms and games separate?

The reason to have separate `games` and `rooms` tables — even if they currently have a **1-to-1 relationship** — is about **design flexibility, semantic clarity**, and **future extensibility**.


| Purpose                 | Reason                                                                                                                                                                                                                                                                                              |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Semantic separation** | A **room** is the *virtual space* where a game happens. A **game** is the *activity* taking place in that room. They serve different conceptual purposes.                                                                                                                                           |
| **Extensibility**       | In the future, you might want: <ul><li>A room to support **different games over time** (e.g., first chess, then checkers)</li><li>A **game to span multiple rooms**, for example, in team or tournament play</li></ul>                                                                              |
| **Metadata separation** | Rooms may contain additional metadata like: <ul><li>Max number of players</li><li>Creation/access control parameters</li><li>Physical display/device info</li></ul> Games store gameplay-specific data like: <ul><li>Game type, ruleset</li><li>Turn state</li><li>Serialized board state</li></ul> |
| **Archival**            | If you ever want to keep a **history of games per room**, separating tables makes it easier. E.g., each room can have a list of past games.                                                                                                                                                         |

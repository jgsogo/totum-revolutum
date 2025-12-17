#include "engine_service.h"

#include <spdlog/spdlog.h>

#include "apps/board_games/engine/data/game.h"
#include "apps/board_games/engine/data/room.h"

namespace services {

    namespace {
        Expected<std::pair<data::Game, std::optional<data::Participant>>>
        get_game_and_participant(pqxx::connection& conn, const data::RoomUUID& room,
                                 const data::ParticipantUUID& participant) {
            SPDLOG_DEBUG("get_game_and_participant(conn, room={}, participant={})", room, participant);
            return data::find_game(conn, room)
                .and_then([&conn, &room, &participant](std::optional<data::Game>&& game_opt)
                              -> Expected<std::pair<data::Game, std::optional<data::Participant>>> {
                    if (game_opt) {
                        data::Game game = std::move(*game_opt);
                        return data::find_participant(conn, room, participant)
                            .and_then(
                                [game = std::move(game)](std::optional<data::Participant>&& participant_opt) mutable
                                    -> Expected<std::pair<data::Game, std::optional<data::Participant>>> {
                                    return Expected<std::pair<data::Game, std::optional<data::Participant>>>{
                                        tl::in_place, std::move(game), std::move(participant_opt)};
                                });
                    } else {
                        return tl::unexpected{errors::InvalidData{std::format("No game found in room {}", room)}};
                    }
                });
        }

        Expected<std::pair<data::Game, data::Participant>>
        get_game_and_participant_required(pqxx::connection& conn, const data::RoomUUID& room,
                                          const data::ParticipantUUID& participant) {
            SPDLOG_DEBUG("get_game_and_participant_required(conn, room={}, participant={})", room, participant);
            return get_game_and_participant(conn, room, participant)
                .and_then([participant_uuid = participant, room_uuid = room](
                              auto&& game_and_participant_opt) -> Expected<std::pair<data::Game, data::Participant>> {
                    auto&& [game, participant_opt] = std::move(game_and_participant_opt);
                    if (!participant_opt) {
                        return tl::unexpected(errors::InvalidData{
                            std::format("No participant found in room {} with uuid {}", room_uuid, participant_uuid)});
                    }

                    Expected<std::pair<data::Game, data::Participant>> r{
                        std::make_pair(std::move(game), std::move(*participant_opt))};
                    return r;
                });
        }

    } // namespace

    EngineServiceImpl::EngineServiceImpl(utils::libpqxx::ConnectionPool& pool, const engine::GamePluginsMap& games)
        : pool{pool}, _games(games) {}

    grpc::Status EngineServiceImpl::CreateNewRoom(grpc::ServerContext* context,
                                                  const board_games::NewRoomRequest* request,
                                                  google::protobuf::Empty* response) {
        SPDLOG_DEBUG("CreateNewRoom");
        return pool.with_conn<grpc::Status>([request](pqxx::connection& conn) -> grpc::Status {
            data::RoomUUID room{std::string{request->uuid()}};
            return data::insert_new_room(conn, room, request->name())
                .and_then([&conn, &room]() { return data::notify_room_update(conn, room); })

                .and_then([]() { return Expected<grpc::Status>{grpc::Status::OK}; })
                .or_else([](const auto& e) {
                    auto status =
                        grpc::Status{grpc::StatusCode::INTERNAL, std::format("Failed to insert new room: {}", e)};
                    return Expected<grpc::Status>{status};
                })
                .value();
        });
    }

    grpc::Status EngineServiceImpl::StartGame(grpc::ServerContext* context,
                                              const board_games::StartGameRequest* request,
                                              google::protobuf::Empty* response) {
        SPDLOG_DEBUG("StartGame");
        return pool.with_conn<grpc::Status>([request, this](pqxx::connection& conn) {
            data::RoomUUID room{std::string{request->room_uuid()}};
            data::GameType game_type{std::string{request->game_type()}};

            // FIXME: remove_game and start_game should go inside the same transaction
            return data::remove_game(conn, room)
                .and_then([&conn, &room, &game_type, this]() -> Expected<void> {
                    auto it = this->_games.find(game_type);
                    if (it == this->_games.end()) {
                        SPDLOG_ERROR("Game type {} not known", game_type);
                        return tl::unexpected{errors::InvalidData{std::format("Game type {} not known", game_type)}};
                    }
                    const auto& game = it->second;
                    return game->new_board().and_then(
                        [&conn, &room, &game_type](data::GamePayload&& initial_board_status) {
                            return data::start_game(conn, room, game_type, initial_board_status);
                        });
                })
                .and_then([&conn, &room]() { return data::notify_room_update(conn, room); })

                .and_then([]() { return Expected<grpc::Status>{grpc::Status::OK}; })
                .or_else([](const auto& e) {
                    auto status = grpc::Status{grpc::StatusCode::INTERNAL, std::format("Failed to start game: {}", e)};
                    return Expected<grpc::Status>{status};
                })
                .value();
        });
    }

    grpc::Status EngineServiceImpl::JoinGame(grpc::ServerContext* context,
                                             const board_games::SendActionRequest* request,
                                             google::protobuf::Empty* response) {
        SPDLOG_DEBUG("JoinGame(request.room_uuid={}, request.participant_uuid={})", request->room_uuid(),
                     request->participant_uuid());

        return pool.with_conn<grpc::Status>([request, this](pqxx::connection& conn) {
            data::RoomUUID room{std::string{request->room_uuid()}};
            data::ParticipantUUID participant{std::string{request->participant_uuid()}};

            return get_game_and_participant(conn, room, participant)
                .and_then([this, request, &conn, &room,
                           &participant](auto&& game_and_participant_opt) -> Expected<void> {
                    auto&& [game, participant_opt] = std::move(game_and_participant_opt);

                    // Get the game plugin
                    auto it = this->_games.find(game.type);
                    if (it == this->_games.end()) {
                        SPDLOG_ERROR("Game type {} not known", game.type);
                        return tl::unexpected{errors::InvalidData{std::format("Game type {} not known", game.type)}};
                    }
                    const auto& game_plugin = it->second;

                    return utils::expected::ok_or(std::move(participant_opt), errors::LogicalError{"unreachable"})
                        // Just widen the error type so we can chain it
                        .transform_error([](auto&& e) -> Expected<data::Participant>::error_type {
                            return Expected<data::Participant>::error_type{std::move(e)};
                        })
                        // Try to create (and join) a new participant
                        .or_else([&conn, &game, &game_plugin, request, &room,
                                  &participant](auto&& _e) -> Expected<data::Participant> {
                            // Get action payload
                            std::vector<std::byte> payload(request->payload().size());
                            std::memcpy(payload.data(), request->payload().data(), request->payload().size());
                            const auto& action_payload = data::ActionPayload{std::move(payload)};

                            return game_plugin->join_game(game.payload, action_payload)
                                .and_then([&conn, &room, &participant, &game,
                                           &action_payload](data::GameJoinResponse&& res) {
                                    return data::add_participant(conn, room, participant, data::ParticipantRole::PLAYER,
                                                                 res.player_number)
                                        .and_then([&conn, &res, &game, &action_payload](
                                                      data::Participant&& p) -> Expected<data::Participant> {
                                            // TODO: This is duplicated. We have the same logic below in SendAction
                                            return data::store_action(conn, game.id, p.uuid, res.action_type,
                                                                      action_payload, true)
                                                .and_then([&conn, &game,
                                                           &res](const std::int64_t& action_id) -> Expected<void> {
                                                    // TODO: Bulk insert
                                                    for (auto&& ev : res.events) {
                                                        auto inserted = data::store_event(conn, game.id, ev.first,
                                                                                          ev.second, action_id);
                                                        // TODO: Notify events, the frontend might want to show
                                                        // animations
                                                        if (!inserted) {
                                                            return inserted;
                                                        }
                                                    }
                                                    return {};
                                                })
                                                .and_then([&conn, &game, &res]() -> Expected<void> {
                                                    return data::update_game_state(conn, game.id, res.new_game_state,
                                                                                   res.new_game_payload);
                                                })
                                                .and_then([p = std::move(p)]() -> Expected<data::Participant> {
                                                    return {std::move(p)};
                                                });
                                        });
                                });
                        })
                        .and_then([](data::Participant&& p) -> Expected<void> {
                            SPDLOG_INFO("Participant {} is successfully added as player {}", p.uuid, p.player_number);
                            return {};
                        });
                })
                // Always return a grpc::Status
                .and_then([]() -> Expected<grpc::Status> { return {grpc::Status::OK}; })
                .or_else([](auto&& e) -> Expected<grpc::Status> {
                    auto status = grpc::Status{grpc::StatusCode::INTERNAL, std::format("Failed to join game: {}", e)};
                    return {status};
                })
                .value();
        });
    }

    grpc::Status EngineServiceImpl::SendAction(grpc::ServerContext* context,
                                               const board_games::SendActionRequest* request,
                                               google::protobuf::Empty* response) {
        SPDLOG_DEBUG("SendAction(request.room_uuid={}, request.participant_uuid={})", request->room_uuid(),
                     request->participant_uuid());
        return pool.with_conn<grpc::Status>([request, this](pqxx::connection& conn) {
            data::RoomUUID room{std::string{request->room_uuid()}};
            data::ParticipantUUID participant{std::string{request->participant_uuid()}};

            return get_game_and_participant_required(conn, room, participant)
                .and_then([&conn, request, this](auto&& game_and_participant) -> Expected<void> {
                    auto&& [game, participant] = std::move(game_and_participant);

                    // We don't care if the game is enabled or not. Maybe it's an ongoing game
                    auto it = this->_games.find(game.type);
                    if (it == this->_games.end()) {
                        SPDLOG_ERROR("Game type {} not known", game.type);
                        return tl::unexpected{errors::InvalidData{std::format("Game type {} not known", game.type)}};
                    }
                    const auto& game_plugin = it->second;
                    std::vector<std::byte> payload(request->payload().size());
                    std::memcpy(payload.data(), request->payload().data(), request->payload().size());
                    const auto& action_payload = data::ActionPayload{std::move(payload)};
                    return game_plugin
                        ->run(game.payload, action_payload, participant.player_number)
                        // Store to the database the action + new status + events
                        // FIXME: We might want to do all of this in a single transaction
                        .and_then([&conn, &game, &participant, &action_payload](data::GameActionResponse&& res) {
                            return data::store_action(conn, game.id, participant.uuid, res.action_type, action_payload,
                                                      true)
                                .and_then([&conn, &game, &res](const std::int64_t& action_id) -> Expected<void> {
                                    // TODO: Bulk insert
                                    for (auto&& ev : res.events) {
                                        auto inserted =
                                            data::store_event(conn, game.id, ev.first, ev.second, action_id);
                                        // TODO: Notify events, the frontend might want to show animations
                                        if (!inserted) {
                                            return inserted;
                                        }
                                    }
                                    return {};
                                })
                                .and_then([&conn, &game, &res]() {
                                    return data::update_game_state(conn, game.id, res.new_game_state,
                                                                   res.new_game_payload);
                                });
                        })
                        // On failure: RETURN to the user that the action could not be understood.
                        .or_else([&conn, &game, &participant, &action_payload](const auto& e) -> Expected<void> {
                            SPDLOG_ERROR("Failed to apply action to the game: {}", e);
                            std::ignore =
                                data::store_action(conn, game.id, participant.uuid, "unknown", action_payload, false);
                            return tl::unexpected{
                                errors::InvalidAction{std::format("Failed to apply action to the game: {}", e)}};
                        });
                })
                // Send notification to the room
                .and_then([&conn, &room]() { return data::notify_room_update(conn, room); })
                // Always return a grpc::Status
                .and_then([]() -> Expected<grpc::Status> { return {grpc::Status::OK}; })
                .or_else([](auto&& e) -> Expected<grpc::Status> {
                    auto status =
                        grpc::Status{grpc::StatusCode::INTERNAL, std::format("Failed to apply action to game: {}", e)};
                    return {status};
                })
                .value();
        });
    }
} // namespace services

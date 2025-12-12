from core.models import Action, Event, Game, GameType, Participant, Room, Snapshot
from django.contrib import admin


class RoomAdmin(admin.ModelAdmin):
    list_display = ["name", "id", "created_at"]
    list_filter = ["created_at", "is_public", "is_open", "updated_at"]

    readonly_fields = ["id", "created_at", "updated_at"]


class GameAdmin(admin.ModelAdmin):
    list_display = ["game_type", "room", "state", "created_at"]
    list_filter = ["game_type", "state", "updated_at"]

    readonly_fields = ["created_at", "updated_at"]


class ParticipantAdmin(admin.ModelAdmin):
    list_display = ["id", "role", "room", "game__game_type"]
    list_filter = ["role", "game__game_type"]

    readonly_fields = ["id"]


class ActionAdmin(admin.ModelAdmin):
    list_display = ["game__room", "game__game_type", "participant__id", "timestamp", "action_type"]
    list_filter = ["game", "action_type"]


class EventAdmin(admin.ModelAdmin):
    list_display = ["game__room", "game__game_type", "timestamp", "event_type"]
    list_filter = ["game", "event_type"]


class SnapshotAdmin(admin.ModelAdmin):
    list_display = ["game__room", "game__game_type", "event__event_type", "timestamp"]
    list_filter = [
        "game",
    ]


admin.site.register(Event, EventAdmin)
admin.site.register(Action, ActionAdmin)
admin.site.register(GameType)
admin.site.register(Game, GameAdmin)
admin.site.register(Participant, ParticipantAdmin)
admin.site.register(Room, RoomAdmin)
admin.site.register(Snapshot, SnapshotAdmin)

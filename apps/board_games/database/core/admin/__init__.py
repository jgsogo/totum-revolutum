from core.models import (
    EventLog,
    Game,
    GameAction,
    GameType,
    Participant,
    Room,
)
from django.contrib import admin


class RoomAdmin(admin.ModelAdmin):
    list_display = ["name", "id", "created_at"]
    list_filter = ["created_at", "is_public", "is_open", "updated_at"]

    readonly_fields = ["id", "created_at", "updated_at"]


class GameAdmin(admin.ModelAdmin):
    list_display = ["__str__", "game_type", "state", "created_at"]
    list_filter = ["game_type", "state", "updated_at"]

    readonly_fields = ["created_at", "updated_at"]


class ParticipantAdmin(admin.ModelAdmin):
    list_display = ["id", "role", "room", "game__game_type"]
    list_filter = ["role", "game__game_type"]

    readonly_fields = ["id"]


admin.site.register(EventLog)
admin.site.register(GameAction)
admin.site.register(GameType)
admin.site.register(Game, GameAdmin)
admin.site.register(Participant, ParticipantAdmin)
admin.site.register(Room, RoomAdmin)

from django.contrib import admin

from apps.board_games.database.core.models import (
    EventLog,
    Game,
    GameType,
    Participant,
    Room,
)

admin.site.register(EventLog)
admin.site.register(GameType)
admin.site.register(Game)
admin.site.register(Participant)
admin.site.register(Room)

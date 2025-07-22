from core.models import (
    EventLog,
    Game,
    GameAction,
    GameType,
    Participant,
    Room,
)
from django.contrib import admin

admin.site.register(EventLog)
admin.site.register(GameAction)
admin.site.register(GameType)
admin.site.register(Game)
admin.site.register(Participant)
admin.site.register(Room)

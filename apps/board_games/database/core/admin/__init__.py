from core.models import (
    EventLog,
    Game,
    GameType,
    Participant,
    Room,
)
from django.contrib import admin

admin.site.register(EventLog)
admin.site.register(GameType)
admin.site.register(Game)
admin.site.register(Participant)
admin.site.register(Room)

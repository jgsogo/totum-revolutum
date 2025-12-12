from django.db import models
from django.utils import timezone

from .event import Event
from .game import Game


class Snapshot(models.Model):
    """
    Stores intermediate states of the game data after GameEvents are applied
    """

    game = models.ForeignKey(
        Game,
        on_delete=models.CASCADE,
        help_text="Game for this snapshot.",
    )

    event = models.ForeignKey(
        Event,
        on_delete=models.CASCADE,
        help_text="Associated event that generated this Snapshot",
    )

    timestamp = models.DateTimeField(default=timezone.now, help_text="When the snapshot was taken.")

    payload = models.BinaryField(
        help_text="Raw binary data of the game state, typically a serialized protobuf."
    )

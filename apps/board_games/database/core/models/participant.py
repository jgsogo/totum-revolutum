import uuid

from django.db import models
from django.utils import timezone

from .game import Game


class Participant(models.Model):
    """
    Represents a player participating in a room.

    **Notes:**
    Player authentication is not managed. Player names are assumed to
    be ephemeral or anonymous initially.
    """

    id = models.UUIDField(
        primary_key=True,
        default=uuid.uuid4,
        editable=False,
        help_text="Unique identifier for the participant.",
    )
    game = models.ForeignKey(
        Game,
        on_delete=models.CASCADE,
        related_name="participants",
        help_text="Game this participant is involved in.",
    )
    role = models.CharField(
        max_length=32, help_text="Role of the participant (e.g., 'player', 'spectator', 'referee')."
    )
    join_time = models.DateTimeField(
        default=timezone.now, help_text="Timestamp when the participant joined the game."
    )

    def __str__(self):
        return f"{self.role} in {self.game}"

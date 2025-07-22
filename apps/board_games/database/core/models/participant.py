import uuid

from django.db import models
from django.utils import timezone

from .game import Game
from .room import Room


class Participant(models.Model):
    """
    Represents a player participating in a room.

    **Notes:**
    Player authentication is not managed. Player names are assumed to
    be ephemeral or anonymous initially.
    """

    class Role(models.TextChoices):
        PLAYER = "player", "Player"
        SPECTATOR = "spectator", "Spectator"
        # REFEREE = "referee", "Referee"

    class Meta:
        unique_together = ("game", "player_number")

    id = models.UUIDField(
        primary_key=True,
        default=uuid.uuid4,
        editable=False,
        help_text="Unique identifier for the participant.",
    )

    room = models.ForeignKey(
        Room, on_delete=models.CASCADE, help_text="Room this participant is currently in."
    )

    role = models.CharField(
        max_length=20,
        choices=Role.choices,
        default=Role.SPECTATOR,
        help_text="Role of the participant (e.g., 'player', 'spectator', 'referee').",
    )

    joined_at = models.DateTimeField(
        default=timezone.now, help_text="Timestamp when the participant joined the game."
    )

    game = models.ForeignKey(
        Game,
        on_delete=models.SET_NULL,
        null=True,
        blank=True,
        help_text="Game this participant played in (if applicable).",
    )

    player_number = models.PositiveSmallIntegerField(
        null=True,
        blank=True,
        help_text="The player's number/seat in the game (e.g., 1 or 2). Null for spectators.",
    )

    def __str__(self):
        return f"{self.role} in {self.game}"

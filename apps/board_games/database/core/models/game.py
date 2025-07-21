import uuid

from django.db import models

from .game_type import GameType
from .room import Room


# Game is a session of a given type in a room
class Game(models.Model):
    """
    Represents a specific game instance (e.g. a chess match between two players).

    **Notes:**
    The `state` column stores the authoritative game state in serialized protobuf format.
    This allows the engine to restore a game at any time.
    """

    id = models.UUIDField(
        primary_key=True,
        default=uuid.uuid4,
        editable=False,
        help_text="Unique identifier for the game instance.",
    )
    room = models.OneToOneField(
        Room, on_delete=models.CASCADE, help_text="Room where this game is being played."
    )
    game_type = models.ForeignKey(
        GameType, on_delete=models.PROTECT, help_text="Type of game being played in this session."
    )
    created_at = models.DateTimeField(
        auto_now_add=True, help_text="Timestamp when the game instance was created."
    )
    current_state = models.BinaryField(
        blank=True, null=True, help_text="Optional serialized game state (protobuf, binary)."
    )

    def __str__(self):
        return f"{self.game_type.name} in {self.room.name}"

from django.db import models

from .game_type import GameType
from .room import Room


# Game is a session of a given type in a room
class Game(models.Model):
    """
    Represents a game instance being played in a room.

    **Notes:**
    - Each game is associated with a `room` and a `game_type`.
    - Games transition through states as players take actions.
    - Game state is serialized in `state_data`.
    """

    class State(models.TextChoices):
        WAITING = "waiting", "Waiting"
        PLAYING = "playing", "Playing"
        FINISHED = "finished", "Finished"

    room = models.OneToOneField(
        Room, on_delete=models.CASCADE, help_text="The room where this game is taking place."
    )

    game_type = models.ForeignKey(
        GameType,
        on_delete=models.PROTECT,
        help_text="The type of game being played (e.g., Checkers, Chess).",
    )

    created_at = models.DateTimeField(
        auto_now_add=True, help_text="Timestamp when the game was created."
    )

    updated_at = models.DateTimeField(
        auto_now=True, help_text="Timestamp when the game was last updated (e.g. a move was made)."
    )

    state = models.CharField(
        max_length=10,
        choices=State.choices,
        default=State.WAITING,
        help_text="State of the game: waiting, playing, or finished.",
    )

    state_data = models.BinaryField(
        help_text="Raw binary data of the action, typically a serialized protobuf."
    )

    def __str__(self):
        return f"{self.game_type.name} in {self.room.name}"

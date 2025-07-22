from django.db import models
from django.utils import timezone

from .game import Game
from .participant import Participant


class GameAction(models.Model):
    """
    Stores the history of moves performed by players in a game.

    **Notes:**
    Each move is stored in protobuf format and can be replayed for auditing or visualization.
    """

    game = models.ForeignKey(
        Game,
        on_delete=models.CASCADE,
        help_text="The game in which this action was performed.",
    )

    participant = models.ForeignKey(
        Participant,
        on_delete=models.SET_NULL,
        null=True,
        blank=True,
        help_text="The participant who performed this action. Can be null for system actions.",
    )

    action_index = models.PositiveIntegerField(
        help_text="Sequential index of the action in the game timeline, starting from 0."
    )

    timestamp = models.DateTimeField(default=timezone.now, help_text="When the action was taken.")

    payload = models.BinaryField(
        help_text="Raw binary data of the action, typically a serialized protobuf."
    )

    class Meta:
        unique_together = ("game", "action_index")
        ordering = ["game", "action_index"]

    def __str__(self):
        return f"Action #{self.action_index} in game {self.game_id}"

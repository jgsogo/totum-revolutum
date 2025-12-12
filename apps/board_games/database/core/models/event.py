from django.db import models
from django.utils import timezone

from .action import Action
from .game import Game


class Event(models.Model):
    """
    Logs notable events, such as game state transitions or system actions.

    **Notes:**
    Used for debugging, analytics, and UI enhancements.
    """

    game = models.ForeignKey(
        Game,
        on_delete=models.CASCADE,
        help_text="Game in which this event occurred.",
    )
    timestamp = models.DateTimeField(
        default=timezone.now, help_text="When the event occurred (according to engine or server)."
    )
    event_type = models.CharField(
        max_length=64, help_text="Type or name of the event (e.g., 'move_made', 'game_over')."
    )
    payload = models.BinaryField(help_text="Serialized payload of the event (e.g., protobuf).")

    action = models.ForeignKey(
        Action,
        on_delete=models.CASCADE,
        help_text="Associated action that generated this event",
    )

    applied = models.BooleanField(
        help_text="If the event has been applied or not",
    )

    def __str__(self):
        return f"Event {self.event_type} at {self.timestamp} in {self.game}"

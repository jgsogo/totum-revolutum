from django.db import models
from django.utils import timezone

from .game import Game


# Events describe changes that occurred during a game
class EventLog(models.Model):
    """
    Logs notable events, such as game state transitions or system actions.

    **Notes:**
    Used for debugging, analytics, and UI enhancements.
    """

    id = models.BigAutoField(
        primary_key=True, help_text="Auto-incrementing ID for ordering events."
    )
    game = models.ForeignKey(
        Game,
        on_delete=models.CASCADE,
        related_name="event_logs",
        help_text="Game in which this event occurred.",
    )
    timestamp = models.DateTimeField(
        default=timezone.now, help_text="When the event occurred (according to engine or server)."
    )
    event_type = models.CharField(
        max_length=64, help_text="Type or name of the event (e.g., 'move_made', 'game_over')."
    )
    payload = models.BinaryField(help_text="Serialized payload of the event (e.g., protobuf).")

    def __str__(self):
        return f"Event {self.event_type} at {self.timestamp} in {self.game}"

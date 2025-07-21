import uuid

from django.db import models


class GameType(models.Model):
    """
    Stores information about the types of games supported by the system (e.g. chess, checkers).

    **Notes:**
    Each game type defines its rules externally in the engine. This table allows
    multi-game support in the same database.
    """

    id = models.UUIDField(
        primary_key=True,
        default=uuid.uuid4,
        editable=False,
        help_text="Unique identifier for the game type (e.g., checkers, chess).",
    )
    name = models.CharField(
        max_length=100, unique=True, help_text="Human-readable name of the game (e.g., Checkers)."
    )
    description = models.TextField(
        blank=True, help_text="Optional description of the game rules or objectives."
    )

    def __str__(self):
        return self.name

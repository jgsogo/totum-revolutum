import uuid

from django.db import models


# Game types define the ruleset (e.g., checkers, chess, go)
class GameType(models.Model):
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

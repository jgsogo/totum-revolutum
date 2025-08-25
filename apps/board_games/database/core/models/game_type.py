from django.db import models


class GameType(models.Model):
    """
    Represents a type of game that can be played in a room.

    Examples: Checkers, Chess, Tic Tac Toe.

    **Notes:**
    - This model allows us to support multiple games with different rulesets.
    - Games in the `Game` table reference a `GameType` via a foreign key.
    """

    slug = models.SlugField(
        primary_key=True,
        max_length=50,
        help_text=(
            "Unique identifier for the game type, used in URLs and references. "
            "Examples: 'checkers', 'chess', 'tic_tac_toe'."
        ),
    )
    name = models.CharField(
        max_length=100,
        help_text="Human-readable name of the game type. Example: 'Checkers'.",
    )
    description = models.TextField(
        blank=True,
        help_text="Optional description of the game type and its rules or characteristics.",
    )

    enabled = models.BooleanField(
        help_text="If the game_type is enabled or not. It won't be possible to play disabled games",
        default=False,
    )

    def __str__(self):
        return self.name

import uuid

from django.db import models


class Room(models.Model):
    """
    Represents a room where players gather and play a game.

    **Notes:**
    A room may only have one game active at a time. The web UI subscribes to
    room events via `LISTEN room_<id>`.

    The `status` field allows distinguishing between rooms in different states
    (e.g., idle, playing, finished). This can be used to filter rooms in the UI.
    """

    id = models.UUIDField(
        primary_key=True,
        default=uuid.uuid4,
        editable=False,
        help_text="Unique identifier for the room.",
    )

    name = models.CharField(max_length=100, help_text="Public name of the room, visible to users.")

    created_at = models.DateTimeField(
        auto_now_add=True, help_text="Timestamp when the room was created."
    )

    updated_at = models.DateTimeField(
        auto_now=True, help_text="Timestamp of the last update to the room (e.g., new participant)."
    )

    is_public = models.BooleanField(
        default=True, help_text="If true, the room is visible to everyone; otherwise, it's private."
    )

    is_open = models.BooleanField(
        default=True, help_text="If false, new participants cannot join this room."
    )

    def __str__(self):
        return self.name

    @property
    def state(self):
        from .game import (
            Game,  # FIXME: Move to some other file so we can import in the header
        )

        return self.game.state if hasattr(self, "game") else Game.State.WAITING

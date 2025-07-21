import uuid

from django.db import models


# A room hosts a single game instance
class Room(models.Model):
    id = models.UUIDField(
        primary_key=True,
        default=uuid.uuid4,
        editable=False,
        help_text="Unique identifier for the room.",
    )
    name = models.CharField(
        max_length=100, unique=True, help_text="Public name of the room, visible to users."
    )
    created_at = models.DateTimeField(
        auto_now_add=True, help_text="Timestamp when the room was created."
    )

    def __str__(self):
        return self.name

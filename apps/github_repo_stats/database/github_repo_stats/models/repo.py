from django.db import models


class Repository(models.Model):
    server = models.CharField()
    org = models.CharField()
    name = models.CharField()

    class Meta:
        unique_together = ("org", "name")

    def __str__(self):
        return f"{self.org}/{self.name}"

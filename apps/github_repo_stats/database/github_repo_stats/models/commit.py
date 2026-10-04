from django.db import models

from .contributor import User
from .repo import Repository


class Commit(models.Model):
    repo = models.ForeignKey(Repository, on_delete=models.CASCADE)

    sha = models.CharField()
    date = models.DateField(blank=True, null=True)

    pull_request = models.IntegerField(blank=True, null=True)

    contributors = models.ManyToManyField(User)
    # reviewers = models.ManyToManyField(User, related_name="commit_reviewed_set")

    changes = models.IntegerField(default=0)

    class Meta:
        unique_together = ("repo", "sha")

    def __str__(self):
        return self.sha

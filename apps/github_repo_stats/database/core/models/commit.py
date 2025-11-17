from django.db import models

from .contributor import Contributor
from .repo import Repository


class Commit(models.Model):
    repo = models.ForeignKey(Repository, on_delete=models.CASCADE)

    sha = models.CharField()
    date = models.DateField()

    pull_request = models.IntegerField(blank=True, null=True)

    contributors = models.ManyToManyField(Contributor)
    reviewers = models.ManyToManyField(Contributor, related_name="commit_reviewed_set")

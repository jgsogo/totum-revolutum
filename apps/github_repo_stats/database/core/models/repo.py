from django.db import models


class Repository(models.Model):
    server = models.CharField()
    org = models.CharField()
    name = models.CharField()

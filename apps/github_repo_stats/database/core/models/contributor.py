from django.db import models


class User(models.Model):
    name = models.CharField()


class Contributor(models.Model):
    user = models.ForeignKey(User, blank=True, null=True, on_delete=models.SET_NULL)
    email = models.EmailField()

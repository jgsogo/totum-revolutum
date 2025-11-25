from django.db import models


class User(models.Model):
    name = models.CharField()

    def __str__(self):
        return self.name


class Contributor(models.Model):
    user = models.ForeignKey(User, blank=True, null=True, on_delete=models.SET_NULL)
    email = models.EmailField(unique=True)

    def __str__(self):
        return self.email

from core.models import Commit, Contributor, Repository, User
from django.contrib import admin

admin.site.register(Repository)
admin.site.register(Commit)
admin.site.register(Contributor)
admin.site.register(User)

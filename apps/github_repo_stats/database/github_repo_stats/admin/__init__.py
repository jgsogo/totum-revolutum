from django.contrib import admin
from github_repo_stats.models import Commit, Contributor, Repository, User


class ContributorAdmin(admin.ModelAdmin):
    list_display = ["email", "user"]
    list_filter = ["user"]


class CommitAdmin(admin.ModelAdmin):
    list_display = ["sha", "changes", "contributors_count"]
    list_filter = ["repo"]

    def contributors_count(self, obj):
        return len(obj.contributors.all())


class UserAdmin(admin.ModelAdmin):
    list_display = ["name", "commit_count"]

    def commit_count(self, obj):
        return len(obj.commit_set.all())


admin.site.register(Repository)
admin.site.register(Commit, CommitAdmin)
admin.site.register(Contributor, ContributorAdmin)
admin.site.register(User, UserAdmin)

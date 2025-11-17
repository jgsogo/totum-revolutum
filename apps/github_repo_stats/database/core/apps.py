from django.apps import AppConfig


class GithubRepoStatsConfig(AppConfig):
    default_auto_field = "django.db.models.BigAutoField"
    name = "core"
    label = "github_repo_stats"
    verbose_name = "GitHub repo stats"

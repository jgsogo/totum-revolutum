from django.apps import AppConfig


class BoardGamesCoreConfig(AppConfig):
    default_auto_field = "django.db.models.BigAutoField"
    name = "apps.board_games.database.core"
    label = "board_games_core"
    verbose_name = "Board Games Core"

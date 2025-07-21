from core.models import GameType
from django.test import TestCase


class GameTypeTestCase(TestCase):
    def test_basic(self):
        game_type = GameType.objects.create(name="game1", description="it's about it")
        self.assertEqual(str(game_type), "game1")

from io import StringIO

from django.core.management import call_command
from django.test import TestCase


class ClosepollTest(TestCase):
    def test_command_output(self):
        out = StringIO()
        call_command(
            "migrate_accountholders",
            DATABASE_URL="//finances_ro@localhost:5432/finances",
            stdout=out,
        )
        self.assertIn('Successfully closed poll "1"', out.getvalue())

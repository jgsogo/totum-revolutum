from datetime import date

from django.core.exceptions import ValidationError
from django.test import TestCase
from finances_data.models import Fx


class FxTestCase(TestCase):
    def test_basic(self):
        fx = Fx.objects.create(local="EUR", foreign="USD", date_value=date(1900, 12, 31), rate=1.25)
        self.assertEqual(str(fx), "EUR/USD = 1.25 (1900-12-31)")
        self.assertEqual(fx.inverse(), 0.8)

    def test_positive_rate(self):
        fx = Fx.objects.create(local="EUR", foreign="USD", date_value=date.today(), rate=-23)
        with self.assertRaises(ValidationError) as cm:
            # Validators are not run when saving the model, see
            # https://docs.djangoproject.com/en/5.1/ref/validators/#how-validators-are-run
            fx.full_clean()

        self.assertListEqual(cm.exception.messages, ["Rate should be equal or greater than 0"])

    def test_different_ccys(self):
        fx = Fx.objects.create(local="USD", foreign="USD", date_value=date.today(), rate=2)
        with self.assertRaises(ValidationError) as cm:
            fx.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Equal local and foreign currencies makes no sense"]
        )

    def test_decimal_places(self):
        fx = Fx.objects.create(local="USD", foreign="EUR", date_value=date.today(), rate=2.12345)
        with self.assertRaises(ValidationError) as cm:
            fx.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Ensure that there are no more than 4 decimal places."]
        )

    def test_default_ordering(self):
        fx1 = Fx.objects.create(local="EUR", foreign="USD", date_value=date(1900, 12, 31), rate=1)
        fx2 = Fx.objects.create(local="EUR", foreign="USD", date_value=date(2000, 12, 31), rate=2)

        fxs = Fx.objects.all()
        self.assertEqual(len(fxs), 2)
        self.assertListEqual(list(fxs), [fx2, fx1])

    def test_duplicate_date(self):
        # We can have two rates for the same currencies for the same date
        Fx.objects.create(local="EUR", foreign="USD", date_value=date(1900, 12, 31), rate=1)
        Fx.objects.create(local="EUR", foreign="USD", date_value=date(1900, 12, 31), rate=2)

        self.assertEqual(Fx.objects.count(), 2)

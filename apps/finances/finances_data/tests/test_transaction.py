from django.test import TestCase


class TransactionTestCase(TestCase):
    def test_movements(self):
        # TODO: All movements sum zero
        pass


class TransactionGroupTestCase(TestCase):
    def test_movements(self):
        # TODO: transaction.group.start <= min(movments.date_value)
        pass

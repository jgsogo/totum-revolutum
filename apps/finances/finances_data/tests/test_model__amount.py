from django.test import TestCase

# TODO: These could be mixins that are incorporated into other tests


class AmountNumerableTestCase(TestCase):
    def test_validation(self):
        # TODO: quantity >= 0
        # TODO: unit_value >= 0
        pass

    def test_get_amount(self):
        pass


class AmountNonNumerableTestCase(TestCase):
    def test_validation(self):
        # TODO: amount >= 0
        # TODO: unit_value >= 0
        pass

    def test_get_amount(self):
        pass

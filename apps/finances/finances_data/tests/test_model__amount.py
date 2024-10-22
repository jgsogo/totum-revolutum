from django.core.exceptions import ValidationError


class AmountNumerableTestsMixin:
    def _create_instance(self, quantity: float, unit_value: float):
        raise NotImplementedError

    def test_amount_validate_unit_value(self):
        instance = self._create_instance(quantity=0, unit_value=-123)
        with self.assertRaises(ValidationError) as cm:
            instance.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Unit value should be equal or greater than 0"]
        )

    def test_amount_validate_quantity(self):
        instance = self._create_instance(quantity=-10, unit_value=0)
        with self.assertRaises(ValidationError) as cm:
            instance.full_clean()

        self.assertListEqual(cm.exception.messages, ["Quantity should be equal or greater than 0"])

    def test_amount_get_amount(self):
        instance = self._create_instance(quantity=10.2, unit_value=0.5)
        self.assertEqual(instance.get_amount(), 5.1)


class AmountNonNumerableTestMixin:
    def _create_instance(self, amount: float):
        raise NotImplementedError

    def test_amount_validate_amount(self):
        instance = self._create_instance(amount=-1)
        with self.assertRaises(ValidationError) as cm:
            instance.full_clean()

        self.assertListEqual(cm.exception.messages, ["Amount should be equal or greater than 0"])

    def test_amount_get_amount(self):
        instance = self._create_instance(amount=10.2)
        self.assertEqual(instance.get_amount(), 10.2)

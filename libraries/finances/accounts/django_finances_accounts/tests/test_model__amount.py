from django.core.exceptions import ValidationError


class AmountTestMixin:
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

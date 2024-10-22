from django.test import TestCase


class SnapshotTestCase(TestCase):
    def test_validation(self):
        # TODO: unique_together
        # TODO: default ordering
        # TODO: 'date_value' between open and close dates of the account
        pass


class SnapshotNonNumerableTestCase(TestCase):
    pass


class SnapshotNumerableTestCase(TestCase):
    pass

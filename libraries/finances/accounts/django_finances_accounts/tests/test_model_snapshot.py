from datetime import date

from django.core.exceptions import ValidationError
from django.db import IntegrityError
from django.db.models import ProtectedError
from django.test import TestCase
from django_finances_accounts.models import (
    Account,
    AccountType,
    Custodian,
    SnapshotNonNumerable,
)

from .test_model__amount import AmountNonNumerableTestMixin


class SnapshotTestCase(TestCase):
    def setUp(self):
        self.acctype = AccountType.objects.create(name="acctype")
        self.custodian = Custodian.objects.create(name="custodian", country="ES")
        self.account = Account.objects.create(
            name="acc1",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
        )


class SnapshotNonNumerableTestCase(AmountNonNumerableTestMixin, SnapshotTestCase):
    def _create_instance(self, amount: float):
        return SnapshotNonNumerable(
            account=self.account,
            date_value=date(1900, 1, 1),
            amount=amount,
        )

    """
    The following tests only run in one of the variants, as they share the source code logic
    """

    def test_validate_unique_together(self):
        self._create_instance(amount=1).save()
        with self.assertRaises(IntegrityError) as cm:
            self._create_instance(amount=1).save()

        self.assertEqual(
            str(cm.exception),
            "UNIQUE constraint failed: finances_accounts_snapshotnonnumerable.account_id,"
            " finances_accounts_snapshotnonnumerable.date_value",
        )

    def test_validate_default_order(self):
        s1 = SnapshotNonNumerable.objects.create(
            account=self.account, date_value=date(1900, 1, 1), amount=1
        )
        s2 = SnapshotNonNumerable.objects.create(
            account=self.account, date_value=date(2000, 1, 1), amount=1
        )

        all_snapshots = SnapshotNonNumerable.objects.all()
        self.assertEqual(len(all_snapshots), 2)
        self.assertListEqual(list(all_snapshots), [s2, s1])

    def test_validate_date_value_open(self):
        # Cannot create a snapshot before the account.open
        with self.assertRaises(ValidationError) as cm:
            s = SnapshotNonNumerable.objects.create(
                account=self.account, date_value=date(1899, 1, 1), amount=1
            )
            s.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Snapshot date_vale cannot be before account is opened"]
        )

    def test_validate_date_value_close(self):
        # Snapshot cannot be posterior to account.close
        closed_account = Account.objects.create(
            name="acc1",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            close=date(2000, 1, 1),
        )
        with self.assertRaises(ValidationError) as cm:
            s = SnapshotNonNumerable.objects.create(
                account=closed_account, date_value=date(2010, 1, 1), amount=1
            )
            s.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Snapshot date_vale cannot be after account is closed"]
        )

    def test_account_protect(self):
        # We cannot remove an account with Snapshots
        SnapshotNonNumerable.objects.create(
            account=self.account, date_value=date(2010, 1, 1), amount=1
        )
        with self.assertRaises(ProtectedError) as cm:
            self.account.delete()

        self.assertEqual(
            str(cm.exception),
            "(\"Cannot delete some instances of model 'Account' because they are referenced"
            " through protected foreign keys: 'SnapshotNonNumerable.account'.\","
            " {<SnapshotNonNumerable: SnapshotNonNumerable object (1)>})",
        )

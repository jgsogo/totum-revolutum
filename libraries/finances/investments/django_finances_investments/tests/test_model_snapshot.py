from datetime import date

from django.core.exceptions import ValidationError
from django.db import IntegrityError
from django.db.models import ProtectedError
from django.test import TestCase
from django_finances_accounts.models import Account, AccountType, Custodian
from django_finances_investments.models import SnapshotNumerable

from .test_model__amount_numerable import AmountNumerableTestsMixin


class SnapshotNumerableTestCase(AmountNumerableTestsMixin, TestCase):

    def setUp(self):
        self.acctype = AccountType.objects.create(name="acctype")
        self.custodian = Custodian.objects.create(name="custodian", country="ES")
        self.account = Account.objects.create(
            name="acc1",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
        )

    def _create_instance(self, quantity: float, unit_value: float):
        return SnapshotNumerable(
            account=self.account,
            date_value=date(1900, 1, 1),
            quantity=quantity,
            unit_value=unit_value,
        )

    def test_validate_unique_together(self):
        self._create_instance(quantity=1, unit_value=0.5).save()
        with self.assertRaises(IntegrityError) as cm:
            self._create_instance(quantity=1, unit_value=0.5).save()

        self.assertEqual(
            str(cm.exception),
            "UNIQUE constraint failed: finances_investments_snapshotnumerable.account_id,"
            " finances_investments_snapshotnumerable.date_value",
        )

    def test_validate_default_order(self):
        s1 = SnapshotNumerable.objects.create(
            account=self.account, date_value=date(1900, 1, 1), quantity=1, unit_value=0.5
        )
        s2 = SnapshotNumerable.objects.create(
            account=self.account, date_value=date(2000, 1, 1), quantity=1, unit_value=0.5
        )

        all_snapshots = SnapshotNumerable.objects.all()
        self.assertEqual(len(all_snapshots), 2)
        self.assertListEqual(list(all_snapshots), [s2, s1])

    def test_validate_date_value_open(self):
        # Cannot create a snapshot before the account.open
        with self.assertRaises(ValidationError) as cm:
            s = SnapshotNumerable.objects.create(
                account=self.account, date_value=date(1899, 1, 1), quantity=1, unit_value=0.5
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
            s = SnapshotNumerable.objects.create(
                account=closed_account, date_value=date(2010, 1, 1), quantity=1, unit_value=0.5
            )
            s.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Snapshot date_vale cannot be after account is closed"]
        )

    def test_account_protect(self):
        # We cannot remove an account with Snapshots
        SnapshotNumerable.objects.create(
            account=self.account, date_value=date(2010, 1, 1), quantity=1, unit_value=0.5
        )
        with self.assertRaises(ProtectedError) as cm:
            self.account.delete()

        self.assertEqual(
            str(cm.exception),
            "(\"Cannot delete some instances of model 'Account' because they are referenced"
            " through protected foreign keys: 'SnapshotNumerable.account'.\","
            " {<SnapshotNumerable: custodian - acc1 @ 2010-01-01>})",
        )

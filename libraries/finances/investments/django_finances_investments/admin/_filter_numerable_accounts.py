from django_finances_accounts.models import Account


class _FilterNumerableAccounts:
    def formfield_for_foreignkey(self, db_field, request, **kwargs):
        if db_field.name == "account":
            kwargs["queryset"] = Account.objects.filter(is_numerable=True)
        return super().formfield_for_foreignkey(db_field, request, **kwargs)


class _FilterNonNumerableAccounts:
    def formfield_for_foreignkey(self, db_field, request, **kwargs):
        if db_field.name == "account":
            kwargs["queryset"] = Account.objects.filter(is_numerable=False)
        return super().formfield_for_foreignkey(db_field, request, **kwargs)

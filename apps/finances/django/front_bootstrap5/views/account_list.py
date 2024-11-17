from django.views.generic import ListView
from django_finances_accounts.models import Account


class AccountList(ListView):
    model = Account
    template_name = "front/account_list.html"

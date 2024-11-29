import { goto } from "$app/navigation";
import type { Account } from "./models/Account";
import type { Holder } from "./models/Holder";


export async function goTo(holder: Holder, account: Account) {
    if (account.is_numerable) {
        await goto(`/holder/${holder.pk}/account/${account.pk}/detail/numerable`);
    } else {
        await goto(`/holder/${holder.pk}/account/${account.pk}/detail`);
    }
  }

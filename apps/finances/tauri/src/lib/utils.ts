import { goto } from "$app/navigation";
import type { Account } from "./models/Account";
import type { Holder } from "./models/Holder";


/**
 * Returns a promise that resolves when Svelte nagivates to the Account detail view.
 * It uses {@link goto} under the hood.
 *
 * Depending on the Account type it will be redirected to different URLs
 * @param {Holder} holder - The holder we are working with
 * @param {Account} account - Account to redirect to
 */
export async function goToAccountDetail(holder: Holder, account: Account) {
    if (account.is_numerable) {
        await goto(`/holder/${holder.pk}/account/${account.pk}/detail/numerable`);
    } else {
        await goto(`/holder/${holder.pk}/account/${account.pk}/detail`);
    }
  }

export async function goToTransactionCreate(holder: Holder, from?: Account, to?: Account) {
    await goto(`/holder/${holder.pk}/transaction/create?from=${from?.pk}&to=${to?.pk}`);
}

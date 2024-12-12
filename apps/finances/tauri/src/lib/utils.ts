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
    await goto(`/holder/${holder.pk}/account/${account.pk}/detail/${account.category}`);
  }

  /**
   *
   * @param holder Returns a promise that resolves when the Svelte navigates to the
   * Transaction create view. It uses {@link goto} under the hood.
   * @param {Account} from - If provided, it prepopulates transaction origin with this account
   * @param {Account} to - If provided, it prepopulates transaction target with this account
   */
export async function goToTransactionCreate(holder: Holder, account: Account, from?: Account, to?: Account) {
    await goto(`/holder/${holder.pk}/account/${account.pk}/transaction/create?from=${from?.pk}&to=${to?.pk}`);
}

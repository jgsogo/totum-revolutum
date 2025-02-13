import { goto } from "$app/navigation";
import type { DateWrapper } from "../../../../../libraries/googleapis/src-js";
import type { Holder, Account } from "../../models/src-js";

/**
 * Returns a promise that resolves when Svelte nagivates to the Account detail view.
 * It uses {@link goto} under the hood.
 *
 * Depending on the Account type it will be redirected to different URLs
 * @param {Holder} holder - The holder we are working with
 * @param {Account} account - Account to redirect to
 */
export async function goToAccountDetail(holder: Holder, account: Account) {
    let account_view = account.is_numerable() ? "numerable_stock" : "default";
    await goto(`/holder/${holder.pk()}/account/${account.pk()}/detail/${account_view}`);
  }

/**
 *
 * @param holder Returns a promise that resolves when the Svelte navigates to the
 * Transaction create view. It uses {@link goto} under the hood.
 * @param {Account} from - If provided, it prepopulates transaction origin with this account
 * @param {Account} to - If provided, it prepopulates transaction target with this account
 */
export async function goToTransactionCreate(holder: Holder, account: Account, from?: Account, to?: Account) {
    let params = new URLSearchParams();
    if (from) {
      params.append("from", from.pk().toString());
    }
    if (to) {
      params.append("to", to.pk().toString());
    }
    await goto(`/holder/${holder.pk()}/account/${account.pk()}/transaction/create?${params}`);
}

/**
 * Rounds a number to the given decimal digits
 *
 * @param {number} num - Number to round
 * @param {number} digits - Number of decimal digits
 * @param {number?} base - Base for the decial digits
 * @returns - The number rounded to the requested digits (using the given base)
 */
export function toFixedNumber(num: number, digits: number = 4, base?: number){
    const pow = Math.pow(base ?? 10, digits);
    return Math.round(num*pow) / pow;
  }


/**
 * Converts the given DateWrapper into a Date
 *
 * @param {DateWrapper} date - Number to round
 * @returns - The the Date object
 */
export function dateWrapper2Date(date: DateWrapper) : Date {
  return new Date(date.year(), date.month() - 1, date.day());
}

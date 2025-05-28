<script lang="ts">
  import { Tooltip, Span } from 'flowbite-svelte';
  import { DateWrapper, Decimal, Money } from '../../../../../../libraries/googleapis/src-js';
  import { get_fx_spot } from '$lib/commands';
  import { AppState, FxQuote, FxQuotePair, MoneyAmount, Movement, MovementDirection, Snapshot } from '../../../models/src-js';

  let {
    app_state,
    money = $bindable(),
    font_mono = false,
    tooltip = false,
  }: {
    app_state: AppState;
    money: Money | Movement | Snapshot | MoneyAmount | undefined;
    font_mono?: boolean;
    tooltip?: boolean;
  } = $props();

  // Get the money value we are going to print
  let [amount, is_negative] = $derived.by(() => {
    let amount = undefined;
    let is_negative = false;
    if (money instanceof Money) {
      amount = money;
    } else if (money instanceof Movement) {
      amount = money.amount();
      is_negative = money.direction() === MovementDirection.Out;
    } else if (money instanceof Snapshot) {
      amount = money.amount().amount();
    } else if (money instanceof MoneyAmount) {
      amount = money.amount();
    }
    return [amount, is_negative];
  });

  // Create the CSS style to apply
  let css_span: string[] = [];
  if (font_mono) {
    css_span.push('font-mono');
  }

  // If tooltip, compute the rate to the base CCY
  async function get_tootip(amount: Money | undefined): Promise<Money | undefined> {
    if (amount === undefined) {
      return undefined;
    }

    const base_ccy = app_state.base_ccy();
    const money_ccy = amount!.currency_code();
    if (base_ccy === money_ccy) {
      return undefined; // No tooltip
    }

    const fx_rate = await get_fx_spot(money_ccy.toString());
    const fx_pair = FxQuotePair.create_from(base_ccy, money_ccy);
    const fx = FxQuote.create_from(DateWrapper.create_from_date(new Date()), fx_pair, Decimal.create_from_number(fx_rate));

    const money_local = fx?.apply_to(amount!)!;
    return money_local;
  }
  let promise = $derived(get_tootip(amount));
</script>

{#if is_negative}
  <Span class={css_span.join(' ')}>({amount ? amount : ''})</Span>
{:else}
  <Span class={css_span.join(' ')}>&nbsp;{amount ? amount : ''}&nbsp;</Span>
{/if}
{#await promise then amount_local}
  {#if amount_local}
    <Tooltip>
      <Span class={css_span.join(' ')}>{amount_local}</Span>
    </Tooltip>
  {/if}
{/await}

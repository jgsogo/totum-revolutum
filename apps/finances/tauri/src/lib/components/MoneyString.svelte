<script lang="ts">
  import { Tooltip, Span } from 'flowbite-svelte';
  import { DateWrapper, Decimal, type Money } from '../../../../../../libraries/googleapis/src-js';
  import { get_fx_spot } from '$lib/commands';
  import { AppState, FxQuote, FxQuotePair } from '../../../models/src-js';

  let {
    app_state,
    money,
    font_mono = false,
    tooltip = false,
  }: {
    app_state: AppState;
    money: Money | undefined;
    font_mono?: boolean;
    tooltip?: boolean;
  } = $props();

  // Create the CSS style to apply
  let css_span: string[] = [];
  if (font_mono) {
    css_span.push('font-mono');
  }

  // If tooltip, compute the rate to the base CCY
  async function get_tootip(): Promise<Money> {
    const base_ccy = app_state.base_ccy();
    const money_ccy = money!.currency_code();
    if (base_ccy === money_ccy) {
      return money!;
    }

    const fx_rate = await get_fx_spot(money_ccy.toString());
    const fx_pair = FxQuotePair.create_from(base_ccy, money_ccy);
    const fx = FxQuote.create_from(DateWrapper.create_from_date(new Date()), fx_pair, Decimal.create_from_number(fx_rate));

    const money_local = fx?.apply_to(money!)!;
    return money_local;
  }
</script>

<Span class={css_span.join(' ')}>{money ? money : ''}</Span>
{#if tooltip && money && money.currency_code() !== app_state.base_ccy()}
  {#await get_tootip() then amount}
    <Tooltip>
      <Span class={css_span.join(' ')}>{amount}</Span>
    </Tooltip>
  {/await}
{/if}

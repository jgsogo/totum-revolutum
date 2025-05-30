<script lang="ts">
  import { Money } from '../../../../../../libraries/googleapis/src-js';
  import { Account, AppState, Movement, MovementDirection } from '../../../models/src-js';
  import MoneyString from './MoneyString.svelte';

  let {
    app_state,
    account,
    movements,
    ...props
  }: {
    app_state: AppState;
    account: Account;
    movements?: Movement[] | undefined;
  } = $props();

  const money = (function () {
    if (movements === undefined) {
      console.log('No movements provided, just return the last snapshot');
      return account.last_snapshot();
    } else {
      // Sum all the movements after the last_snapshot
      const last_snapshot = account.last_snapshot();
      console.log(`Last snapshot date-value: ${last_snapshot?.date_value()}`);
      const movements_after_last_snapshot = movements.filter((v: Movement) => {
        console.log(` - mov date_value: ${v.date_value()}`);
        return !last_snapshot || last_snapshot.date_value().less_than(v.date_value());
      });
      console.log(`Found ${movements_after_last_snapshot.length} after last snapshot (out of ${movements.length} movs)!`);
      const initial_amount: Money = last_snapshot ? last_snapshot.amount().amount() : Money.create_from_number(account.ccy(), 0);

      console.log(`Initial amount: ${initial_amount}`);
      return movements_after_last_snapshot.reduce((sum: Money, v: Movement) => {
        switch (v.direction()) {
          case MovementDirection.In:
            console.log(` + plus ${v.amount()}`);
            return sum.sum(v.amount());
          case MovementDirection.Out:
            console.log(` + minus ${v.amount()}`);
            return sum.substract(v.amount());
        }
      }, initial_amount);
    }
  })();
</script>

<MoneyString {app_state} {money} {...props} />

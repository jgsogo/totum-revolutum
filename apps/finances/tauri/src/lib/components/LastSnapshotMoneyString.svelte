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
      return account.last_snapshot();
    } else {
      // Sum all the movements after the last_snapshot
      const last_snapshot = account.last_snapshot();
      const movements_after_last_snapshot = movements.filter((v: Movement) => {
        return !last_snapshot || last_snapshot.date_value().less_than(v.date_value());
      });
      const initial_amount: Money = last_snapshot ? last_snapshot.amount().amount() : Money.create_from_number(account.ccy(), 0);

      return movements_after_last_snapshot.reduce((sum: Money, v: Movement) => {
        switch (v.direction()) {
          case MovementDirection.In:
            return sum.sum(v.amount());
          case MovementDirection.Out:
            return sum.substract(v.amount());
        }
      }, initial_amount);
    }
  })();
</script>

<MoneyString {app_state} {money} {...props} />

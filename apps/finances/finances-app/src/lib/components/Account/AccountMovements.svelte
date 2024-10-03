<script lang="ts">
  import {
    TableHandler,
    Datatable,
    ThSort,
    ThFilter,
  } from "@vincjo/datatables";

  // FIXME: See issue https://github.com/vincjo/datatables/issues/132, we need to pin the dependency

  export let data;
  const table = new TableHandler(data, { rowsPerPage: 10 });

</script>

<Datatable basic {table}>
    <table>
        <thead>
            <tr>
                <ThSort {table} field="date_value">Date value</ThSort>
                <ThSort {table} field="amount">Amount</ThSort>
                <ThSort {table} field="ccy">Ccy</ThSort>
            </tr>
            <tr>
                <ThFilter {table} field="date_value"/>
                <ThFilter {table} field="amount" />
                <ThFilter {table} field="ccy"/>
            </tr>
        </thead>
        <tbody>
            {#each table.rows as row}
                <tr>
                    <td>{row.date_value}</td>
                    <td>{row.amount}</td>
                    <td>{row.ccy}</td>
                </tr>
            {/each}
        </tbody>
    </table>
</Datatable>

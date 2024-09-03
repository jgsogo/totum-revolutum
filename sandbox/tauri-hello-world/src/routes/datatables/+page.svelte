<script lang="ts">
	import {
		Datatable,
		functionCreateDatatable,
		PaginationItems,
		RowsPerPage,
		Search,
		Sort,
	} from 'svelte-datatables-net';
    import { invoke } from "@tauri-apps/api/core";

    let stateDatatable = $state();

    const functionReadData = async function () {
        const data = await invoke("datatables_table");
        console.log(data);
        const arrayData = data.map(function(e) {
            return {id: e[0], name: e[1], age: e[2], city: e[3]};
        });
        console.log(arrayData);

		stateDatatable = functionCreateDatatable({
			parData: arrayData,
			parSearchableColumns: ['name', 'city'],
			parRowsPerPage: '10',
			parSortBy: 'city',
			parSearchString: '',
			parSortOrder: 'ascending',
		});
	};

</script>

<svelte:head>
	<link
		href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css"
		rel="stylesheet"
		integrity="sha384-EVSTQN3/azprG1Anm3QDgpJLIm9Nao0Yz1ztcQTwFspd3yD65VohhpuuCOmLASjC"
		crossorigin="anonymous"
	/>
</svelte:head>


<h1>eDatatables example <small>(<a href="/">home</a>)</small></h1>
<p>Using <a href="https://github.com/joaquimnetocel/svelte-datatables-net/tree/master" target="_blank">svelte-datatables-net</a></p>

{#await functionReadData()}
READING DATA...
{:then}
<Datatable bind:propDatatable={stateDatatable}>
	<div class="container-sm">
		<div class="mx-3">
			<!-- SEARCH & RESULTS PER PAGE -->
			<div class="row align-items-center mb-2">
				<div class="col-12 col-md-6 text-md-start text-center mb-1 mb-md-0">
					<div class="d-md-flex align-items-md-center">
						<span class="me-1">Search:</span>
						<Search propPlaceholder="Type here..." class="form-control form-control-sm" />
					</div>
				</div>
				<div class="col-12 col-md-6 text-md-end text-center">
					<RowsPerPage class="d-inline form-select form-select-sm w-auto">
						<option value="5">5</option>
						<option value="10">10</option>
						<option value="20">20</option>
						<option value="30">30</option>
						<option value="all">ALL</option>
					</RowsPerPage>
					<span>RESULTS PER PAGE</span>
				</div>
			</div>
			<!---->
			<!-- PAGINATION -->
			<div class="d-flex justify-content-center justify-content-md-end">
				<nav aria-label="Page navigation example">
					<ul class="pagination">
						<PaginationItems
							propTag="li"
							class="page-item"
							propInnerClass="page-link"
							propDisabledClass="disabled"
							propActiveClass="active"
						/>
					</ul>
				</nav>
			</div>
			<!---->
			{#if stateDatatable.arraySearched.length === 0}
				<div class="text-center mt-5"><strong>NO RECORDS FOUND.</strong></div>
			{:else}
				<!-- TABLE -->
				<table class="table table-striped table-sm">
					<thead>
						<tr>
							<th>
								<Sort propDatatable={stateDatatable} propColumn={'id'}>ID</Sort>
							</th>
							<th>
								<Sort propDatatable={stateDatatable} propColumn={'name'}>NAME</Sort>
							</th>
							<th>AGE</th>
							<th>CITY</th>
						</tr>
					</thead>
					<tbody>
						{#each stateDatatable.arrayData as row}
							<tr>
								<td>{row.id}</td>
								<td>{row.name}</td>
								<td>{row.age}</td>
								<td>{row.city}</td>
							</tr>
						{/each}
					</tbody>
				</table>
				<!---->
			{/if}
			<div>
				SHOWING {stateDatatable.numberFirstRow} TO {stateDatatable.numberLastRow} OF {stateDatatable
					.arraySearched.length} ITEMS
			</div>
		</div>
	</div>
</Datatable>
{/await}

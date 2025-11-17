<script context="module" lang="ts">
	export type ColumnInfo = { name: string; type: string };
	export type DataRow = Record<string, string>;
</script>

<script lang="ts">
	// As props 'schema' e 'rows' usam os tipos
	// que agora existem no escopo do módulo.
	export let schema: ColumnInfo[] = [];
	export let rows: DataRow[] = [];
</script>

<div class="table-wrapper">
	<table class="data-table">
		<thead>
			<tr>
				{#each schema as col}
					<th>{col.name} ({col.type})</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each rows as row}
				<tr>
					{#each schema as col}
						<td>{row[col.name]}</td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	.table-wrapper {
		overflow: auto;
		flex: 1;
		min-height: 0;
		border: 1px solid #ddd;
		border-radius: 4px;
	}
	.data-table {
		width: 100%;
		border-collapse: collapse;
		text-align: left;
		white-space: nowrap;
	}
	.data-table th,
	.data-table td {
		border-bottom: 1px solid #eee;
		border-right: 1px solid #eee;
		padding: 0.5em 1em;
	}
	.data-table th {
		background-color: #f9f9f9;
		position: sticky;
		top: 0;
	}
	.data-table tr:nth-child(even) {
		background-color: #fdfdfd;
	}
</style>
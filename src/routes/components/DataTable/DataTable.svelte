<script context="module" lang="ts">
	export type ColumnInfo = { name: string; type: string };
	export type DataRow = Record<string, string>;
</script>

<script lang="ts">
	export let schema: ColumnInfo[] = [];
	export let rows: DataRow[] = [];
    // NOVO: Recebe o número inicial para calcular a linha correta (ex: 51, 52...)
    export let startRowIndex: number = 0;
</script>

<div class="table-wrapper">
	<table class="data-table">
		<thead>
			<tr>
                <th class="row-number-col">#</th>
				
                {#each schema as col}
					<th>
                        {col.name}
                        <span style="opacity: 0.5; font-weight: normal; margin-left: 4px;">({col.type})</span>
                    </th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each rows as row, i}
				<tr>
                    <td class="row-number-col">{startRowIndex + i + 1}</td>

					{#each schema as col}
						<td title={row[col.name]}>{row[col.name]}</td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style src="./DataTable.css"></style>
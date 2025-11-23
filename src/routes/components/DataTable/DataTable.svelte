<script context="module" lang="ts">
	export type ColumnInfo = { name: string; type: string };
	export type DataRow = Record<string, string>;
</script>

<script lang="ts">
	export let schema: ColumnInfo[] = [];
	export let rows: DataRow[] = [];
    export let startRowIndex: number = 0;

    export let sortCol: string | null = null;
    export let sortOrder: 'ASC' | 'DESC' | null = null;
    
    export let onsort: (col: string) => void = () => {};
</script>

<div class="table-wrapper">
	<table class="data-table">
		<thead>
			<tr>
                <th class="row-number-col">#</th>
				
                {#each schema as col}
                    <th class="sortable-header" on:click={() => onsort(col.name)}>
                        <div class="th-content">
                            <span>{col.name}</span>
                            
                            {#if sortCol === col.name}
                                <span class="sort-icon active">
                                    {#if sortOrder === 'ASC'}
                                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M12 19V5"/><path d="M5 12l7-7 7 7"/></svg>
                                    {:else}
                                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="M19 12l-7 7-7-7"/></svg>
                                    {/if}
                                </span>
                            {:else}
                                <span class="sort-icon ghost">
                                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M7 15l5 5 5-5"/><path d="M7 9l5-5 5 5"/></svg>
                                </span>
                            {/if}
                        </div>
                        
                        <div class="col-type">({col.type})</div>
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
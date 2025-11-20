<script lang="ts">
	import { slide } from 'svelte/transition';
	import type { ColumnInfo } from '../DataTable/DataTable.svelte';

	export let filePath: string = '';
	export let totalRows: number = 0;
	export let schema: ColumnInfo[] = [];

	let isOpen = false;

	function toggle() {
		isOpen = !isOpen;
	}

	$: formattedRows = new Intl.NumberFormat('pt-BR').format(totalRows);
	$: fileName = filePath.split(/[/\\]/).pop() || 'Arquivo Parquet';
</script>

<div class="meta-panel">
	<button class="meta-header" on:click={toggle} aria-expanded={isOpen}>
		<div class="header-left">
			<span class="icon-file">
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/></svg>
			</span>
			<span class="filename">{fileName}</span>
			
			{#if !isOpen}
				<span class="summary-badge" transition:slide={{ axis: 'x', duration: 200 }}>
					{formattedRows} linhas
				</span>
			{/if}
		</div>
		
		<span class="icon-chevron" class:rotated={isOpen}>
			<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"/></svg>
		</span>
	</button>

	{#if isOpen}
		<div class="meta-content" transition:slide={{ duration: 300 }}>
			<div class="info-grid">
				
				<div class="info-item full-width">
                    <div class="meta-label">Caminho do Arquivo</div>
					<code class="path" title={filePath}>{filePath}</code>
				</div>

				<div class="info-card">
					<div class="stat-value">{formattedRows}</div>
					<div class="stat-label">Linhas Totais</div>
				</div>

				<div class="info-card">
					<div class="stat-value">{schema.length}</div>
					<div class="stat-label">Colunas</div>
				</div>
			</div>

			<div class="schema-section">
				<div class="meta-label">Estrutura (Schema)</div>
				<div class="schema-tags">
					{#each schema as col}
						<span class="col-tag" title={col.type}>
							<span class="col-name">{col.name}</span>
							<span class="col-type">{col.type}</span>
						</span>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>

<style src="./MetadataPanel.css"></style>
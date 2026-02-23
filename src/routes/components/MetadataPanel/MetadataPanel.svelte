<script lang="ts">
	import { slide } from 'svelte/transition';
	import type { ColumnInfo, ParquetFileInfo, DataSource } from '$lib/types';

	interface Props {
		sourcePath?: string;
		totalRows?: number;
		schema?: ColumnInfo[];
		dataSource?: DataSource;
		files?: ParquetFileInfo[];
	}

	let { 
		sourcePath = '', 
		totalRows = 0, 
		schema = [], 
		dataSource = 'file',
		files = []
	}: Props = $props();

	let isOpen = $state(false);
	let isMultiFileMode = $derived(dataSource === 'directory' || dataSource === 'fileList');

	let formattedRows = $derived(new Intl.NumberFormat('pt-BR').format(totalRows));
	let sourceName = $derived(
		dataSource === 'fileList' 
			? 'Lista de Arquivos'
			: sourcePath 
				? sourcePath.split(/[/\\]/).pop() || 'Arquivo Parquet'
				: 'Arquivo Parquet'
	);
</script>

<div class="meta-panel">
	<button class="meta-header" onclick={() => isOpen = !isOpen} aria-expanded={isOpen}>
		<div class="header-left">
			<span class="icon-file">
				{#if isMultiFileMode}
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
					</svg>
				{:else}
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/>
						<polyline points="14 2 14 8 20 8"/>
					</svg>
				{/if}
			</span>
			<span class="filename">{sourceName}</span>
			
			{#if !isOpen}
				<span class="summary-badge" transition:slide={{ axis: 'x', duration: 200 }}>
					{#if isMultiFileMode}
						{files.length} arquivos · {formattedRows} linhas
					{:else}
						{formattedRows} linhas
					{/if}
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
				{#if dataSource === 'fileList'}
					<div class="info-item full-width">
						<div class="meta-label">Arquivos Selecionados</div>
						<div class="files-preview">
							{#each files.slice(0, 3) as file}
								<code class="path-mini" title={file.file_path}>{file.file_name}</code>
							{/each}
							{#if files.length > 3}
								<span class="more-files">+{files.length - 3} mais</span>
							{/if}
						</div>
					</div>
				{:else}
					<div class="info-item full-width">
						<div class="meta-label">{dataSource === 'directory' ? 'Caminho da Pasta' : 'Caminho do Arquivo'}</div>
						<code class="path" title={sourcePath}>{sourcePath}</code>
					</div>
				{/if}

				<div class="info-card">
					<div class="stat-value">{formattedRows}</div>
					<div class="stat-label">Linhas Totais</div>
				</div>

				{#if isMultiFileMode}
					<div class="info-card">
						<div class="stat-value">{files.length}</div>
						<div class="stat-label">Arquivos</div>
					</div>
				{:else}
					<div class="info-card">
						<div class="stat-value">{schema.length}</div>
						<div class="stat-label">Colunas</div>
					</div>
				{/if}
			</div>

			{#if isMultiFileMode && files.length > 0}
				<div class="files-section">
					<div class="meta-label">Arquivos ({files.length})</div>
					<div class="files-list">
						{#each files as file}
							<div class="file-item" title={file.file_path}>
								<span class="file-icon">
									<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
										<polyline points="13 2 13 9 20 9"/>
									</svg>
								</span>
								<span class="file-name">{file.file_name}</span>
								<span class="file-rows">{new Intl.NumberFormat('pt-BR').format(file.row_count)} linhas</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}

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

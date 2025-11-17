<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';

	// Importar os novos componentes
	import Titlebar from './Titlebar.svelte';
	import Pagination from './Pagination.svelte';
	import DataTable, { type ColumnInfo, type DataRow } from './DataTable.svelte';

	let isDark = false;

    $: if (typeof document !== 'undefined') {
        document.body.classList.toggle('dark-mode', isDark);
    }
    
    function toggleTheme() {
        isDark = !isDark;
    }

	// --- Tipos de Dados ---
	interface FileMetadata {
		file_path: string;
		total_rows: number;
		schema: ColumnInfo[];
	}

	// --- Estado da Aplicação (Svelte 4) ---
	let schema: ColumnInfo[] = [];
	let rows: DataRow[] = [];
	let errorMsg = '';
	let isLoading = false;
	let selectedFile = '';
	let totalRows = 0;
	let currentPage = 0;
	const pageSize = 50;

	// --- Valores Derivados (Svelte 4) ---
	$: totalPages = Math.ceil(totalRows / pageSize);
	$: hasData = schema.length > 0;

	// --- Lógica de Dados (idêntica) ---
	async function handleOpenFile() {
		isLoading = true;
		errorMsg = '';
		schema = [];
		rows = [];
		selectedFile = '';
		currentPage = 0;
		totalRows = 0;

		try {
			const file = await open({
				title: 'Selecione um arquivo Parquet',
				multiple: false,
				filters: [{ name: 'Parquet', extensions: ['parquet'] }]
			});

			if (file && typeof file === 'string') {
				selectedFile = file;
				const metadata: FileMetadata = await invoke('get_file_metadata', {
					filePath: file
				});

				schema = metadata.schema;
				totalRows = metadata.total_rows;
				if (schema.length > 0) {
					await loadPage(0);
				}
			}
		} catch (e) {
			console.error('Erro ao carregar metadados:', e);
			errorMsg = e as string;
		} finally {
			isLoading = false;
		}
	}

	async function loadPage(page: number) {
		if (!selectedFile) return;
		isLoading = true;
		errorMsg = '';

		try {
			rows = await invoke('get_page_data', {
				filePath: selectedFile,
				page: page,
				pageSize: pageSize
			});
			currentPage = page;
		} catch (e) {
			console.error('Erro ao carregar página:', e);
			errorMsg = e as string;
		} finally {
			isLoading = false;
		}
	}
</script>

<main class="container">
	<Titlebar on:openfile={handleOpenFile} {isDark} on:toggle={toggleTheme} />

	<div class="content">
		<button on:click={handleOpenFile} disabled={isLoading}>
			{isLoading ? 'Carregando...' : 'Abrir Arquivo Parquet'}
		</button>

		{#if errorMsg}
			<p class="error">Erro: {errorMsg}</p>
		{/if}

		{#if hasData}
			<div class="metadata">
				<span><strong>Arquivo:</strong> {selectedFile}</span>
				<span><strong>Total de Linhas:</strong> {totalRows}</span>
			</div>

			<Pagination
				{currentPage}
				{totalPages}
				{isLoading}
				rowsLength={rows.length}
				{pageSize}
				on:prev={() => loadPage(currentPage - 1)}
				on:next={() => loadPage(currentPage + 1)}
			/>

			<DataTable {schema} {rows} />
		{/if}
	</div>
</main>

<style>
    :global(body) {
        --color-background: #ffffff;
        --color-text: #333333;
        --color-subtle-text: #666666;
        --color-border: #eeeeee;
        --color-hover: #e5e5e5;
        --color-titlebar-bg: #f7f7f7;
        --color-metadata-bg: #f4f4ff;
        --color-table-row-even: #fdfdfd;
        --color-table-header-bg: #f9f9f9;
        
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell,
            'Open Sans', 'Helvetica Neue', sans-serif;
        margin: 0;
        padding: 0;
        overflow: hidden;
        background-color: var(--color-background); /* Usando variável */
    }

    :global(body.dark-mode) {
        /* Tema ESCURO (Substituindo variáveis) */
        --color-background: #1e1e1e;
        --color-text: #cccccc;
        --color-subtle-text: #999999;
        --color-border: #444444;
        --color-hover: #3c3c3c;
        --color-titlebar-bg: #252526;
        --color-metadata-bg: #303030;
        --color-table-row-even: #212121;
        --color-table-header-bg: #2d2d2d;
    }

    /* --- 2. ESTILOS GLOBAIS --- */
    :global(html),
    :global(body) {
        height: 100%;
    }

    ::-webkit-scrollbar {
        width: 8px;
        height: 8px;
    }
    ::-webkit-scrollbar-track {
        background: var(--color-background);
    }
    ::-webkit-scrollbar-thumb {
        background: var(--color-subtle-text);
        border-radius: 4px;
    }
    ::-webkit-scrollbar-thumb:hover {
        background: var(--color-text);
    }

    :global(button:focus),
    :global(button:focus-visible) {
        outline: none;
        box-shadow: 0 0 0 2px rgba(0, 120, 255, 0.5);
    }

    /* --- 3. ESTILOS DA PÁGINA (Usando Variáveis) --- */
    .container {
        display: flex;
        flex-direction: column;
        height: 100%;
        min-height: 0;
        padding: 0;
        margin: 0;
        background-color: var(--color-background); /* Usando variável */
        box-sizing: border-box;
    }

    .content {
        padding: 1.5em;
        display: flex;
        flex-direction: column;
        flex: 1;
        min-height: 0;
    }

    .error {
        color: red;
        margin-top: 1em;
        text-align: left;
    }

    .metadata {
        text-align: left;
        color: var(--color-text); /* Usando variável */
        background-color: var(--color-metadata-bg); /* Usando variável */
        padding: 0.5em 1em;
        border-radius: 4px;
        margin-top: 1.5em;
        display: flex;
        justify-content: space-between;
        flex-shrink: 0;
    }
</style>
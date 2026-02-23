<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open, save } from '@tauri-apps/plugin-dialog';
	import { listen } from '@tauri-apps/api/event';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import { onMount, onDestroy } from 'svelte';
	
	import { APP_INFO } from '$lib/constants';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { fileStore } from '$lib/stores/file.svelte';
	import { uiStore } from '$lib/stores/ui.svelte';
	import type { ExportFormat, ExportScope } from '$lib/types';
	
	import Titlebar from './components/Titlebar/Titlebar.svelte';
	import Pagination from './components/Pagination/Pagination.svelte';
	import DataTable from './components/DataTable/DataTable.svelte';
	import MetadataPanel from './components/MetadataPanel/MetadataPanel.svelte';
	import Modal from './components/Modal/Modal.svelte';
	import SqlModal from './components/SqlModal/SqlModal.svelte';
	import ExportModal from './components/ExportModal/ExportModal.svelte';

	let unlistenDrop: (() => void) | null = null;
	let unlistenHover: (() => void) | null = null;
	let unlistenCancel: (() => void) | null = null;

	onMount(async () => {
		await themeStore.init();
		await fileStore.loadRecentFiles();

		unlistenDrop = await listen('tauri://drag-drop', (event: unknown) => {
			const paths = (event as { payload: { paths: string[] } }).payload.paths;
			uiStore.setDragging(false);
			if (paths && paths.length > 0) {
				const parquetFiles = paths.filter(p => p.endsWith('.parquet'));
				const directories = paths.filter(p => !p.endsWith('.parquet'));
				
				if (parquetFiles.length > 1) {
					fileStore.loadFileList(parquetFiles);
				} else if (parquetFiles.length === 1 && directories.length === 0) {
					fileStore.loadFile(parquetFiles[0]);
				} else if (directories.length === 1 && parquetFiles.length === 0) {
					fileStore.loadDirectory(directories[0]);
				} else if (parquetFiles.length > 0) {
					fileStore.loadFileList(parquetFiles);
				}
			}
		});

		unlistenHover = await listen('tauri://drag-enter', () => {
			uiStore.setDragging(true);
		});

		unlistenCancel = await listen('tauri://drag-leave', () => {
			uiStore.setDragging(false);
		});
	});

	onDestroy(() => {
		if (unlistenDrop) unlistenDrop();
		if (unlistenHover) unlistenHover();
		if (unlistenCancel) unlistenCancel();
	});

	async function handleOpenFile(): Promise<void> {
		try {
			const file = await open({
				title: 'Selecione um arquivo Parquet',
				multiple: false,
				filters: [{ name: 'Parquet', extensions: ['parquet'] }]
			});

			if (file && typeof file === 'string') {
				await fileStore.loadFile(file);
			}
		} catch (e) {
			console.error(e);
		}
	}

	async function handleOpenDirectory(): Promise<void> {
		try {
			const dir = await open({
				title: 'Selecione uma pasta com arquivos Parquet',
				directory: true,
				multiple: false
			});

			if (dir && typeof dir === 'string') {
				await fileStore.loadDirectory(dir);
			}
		} catch (e) {
			console.error(e);
		}
	}

	async function handleOpenFiles(): Promise<void> {
		try {
			const files = await open({
				title: 'Selecione arquivos Parquet',
				multiple: true,
				filters: [{ name: 'Parquet', extensions: ['parquet'] }]
			});

			if (files) {
				const paths = Array.isArray(files) ? files : [files];
				if (paths.length === 1) {
					await fileStore.loadFile(paths[0]);
				} else if (paths.length > 1) {
					await fileStore.loadFileList(paths);
				}
			}
		} catch (e) {
			console.error(e);
		}
	}

	async function openExternal(url: string): Promise<void> {
		try {
			await openUrl(url);
		} catch (e) {
			console.error(e);
		}
	}

	function handleExternalClick(e: MouseEvent, url: string): void {
		e.preventDefault();
		openExternal(url);
	}

	function handleRunSql(query: string): void {
		uiStore.closeSql();
		fileStore.runSql(query);
	}

	async function handleExport(format: ExportFormat, scope: ExportScope): Promise<void> {
		if (!fileStore.sourcePath && !fileStore.filePaths) return;
		uiStore.setExporting(true);

		try {
			const ext = format.toLowerCase();
			const savePath = await save({
				title: 'Salvar Arquivo Exportado',
				defaultPath: `export.${ext}`,
				filters: [{ name: format, extensions: [ext] }]
			});

			if (!savePath) {
				uiStore.setExporting(false);
				return;
			}

			let queryToExport = 'SELECT * FROM t';
			if (scope === 'query' && fileStore.currentSqlQuery) {
				queryToExport = fileStore.currentSqlQuery;
			} else if (scope === 'all' && !fileStore.isSqlMode && fileStore.sort.column && fileStore.sort.order) {
				queryToExport += ` ORDER BY "${fileStore.sort.column}" ${fileStore.sort.order}`;
			}

			await invoke('export_data', {
				sourcePath: fileStore.sourcePath ?? '',
				query: queryToExport,
				outputPath: savePath,
				format,
				filePaths: fileStore.filePaths
			});

			uiStore.closeExport();
			alert(`Dados exportados com sucesso para ${savePath}!`);
		} catch (e) {
			console.error('Erro na exportação:', e);
		} finally {
			uiStore.setExporting(false);
		}
	}
</script>

<main class="container">
	<Titlebar onopenfile={handleOpenFile} onopenfiles={handleOpenFiles} onopendirectory={handleOpenDirectory} />

	<div class="content">
		{#if uiStore.isDragging}
			<div class="drop-overlay">
				<div class="drop-message">
					<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
						<polyline points="17 8 12 3 7 8"/>
						<line x1="12" y1="3" x2="12" y2="15"/>
					</svg>
					<span>Solte arquivos ou pastas Parquet aqui</span>
				</div>
			</div>
		{/if}

		{#if !fileStore.hasData && !fileStore.error}
			<div class="empty-state-container">
				<img src="/welcome.svg" alt="Bem-vindo ao OpenParquet" class="welcome-image" />
				<h2 class="welcome-title">{APP_INFO.name}</h2>
				<p class="welcome-subtitle">Arraste arquivos/pasta ou clique abaixo para começar.</p>

				<div class="open-buttons">
					<button class="btn-primary large-btn" onclick={handleOpenFile} disabled={fileStore.isLoading}>
						{#if fileStore.isLoading}
							<span class="loader"></span> Carregando...
						{:else}
							<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right:8px">
								<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
								<polyline points="17 8 12 3 7 8"/>
								<line x1="12" y1="3" x2="12" y2="15"/>
							</svg>
							Abrir Arquivo
						{/if}
					</button>

					<button class="btn-secondary large-btn" onclick={handleOpenFiles} disabled={fileStore.isLoading}>
						<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right:8px">
							<path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
							<polyline points="13 2 13 9 20 9"/>
							<line x1="9" y1="13" x2="15" y2="13"/>
						</svg>
						Múltiplos Arquivos
					</button>

					<button class="btn-secondary large-btn" onclick={handleOpenDirectory} disabled={fileStore.isLoading}>
						<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right:8px">
							<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
						</svg>
						Abrir Pasta
					</button>
				</div>

				{#if fileStore.recentFiles.length > 0}
					<div class="recent-files">
						<h3>Recentes</h3>
						<ul>
							{#each fileStore.recentFiles as path}
								<li>
									<button class="btn-recent" onclick={() => {
										if (path.includes('|')) {
											const files = path.split('|');
											fileStore.loadFileList(files);
										} else if (path.endsWith('.parquet')) {
											fileStore.loadFile(path);
										} else {
											fileStore.loadDirectory(path);
										}
									}}>
										<span class="icon-file">
											{#if path.includes('|')}
												<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
													<path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
													<polyline points="13 2 13 9 20 9"/>
													<line x1="9" y1="13" x2="15" y2="13"/>
												</svg>
											{:else if path.endsWith('.parquet')}
												<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
													<path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
													<polyline points="13 2 13 9 20 9"/>
												</svg>
											{:else}
												<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
													<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
												</svg>
											{/if}
										</span>
										<span class="file-path" title={path}>
											{#if path.includes('|')}
												{path.split('|').length} arquivos
											{:else}
												{path}
											{/if}
										</span>
									</button>
								</li>
							{/each}
						</ul>
					</div>
				{/if}
			</div>
		{/if}

		{#if fileStore.error}
			<div class="actions-bar">
				<div class="error-banner">
					<strong>Erro:</strong> {fileStore.error}
				</div>
				<button class="btn-primary" onclick={handleOpenFile}>Tentar Outro Arquivo</button>
			</div>
		{/if}

		{#if fileStore.hasData}
			<div class="data-header-row">
				<div class="data-toolbar">
					<button class="btn-tool" onclick={() => uiStore.openExport()} title="Exportar Dados" disabled={fileStore.isLoading}>
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
							<polyline points="7 10 12 15 17 10"/>
							<line x1="12" y1="15" x2="12" y2="3"/>
						</svg>
						<span>Exportar</span>
					</button>

					{#if !fileStore.isSqlMode}
						<button class="btn-tool" onclick={() => uiStore.openSql()} title="Executar SQL">
							<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"/>
								<path d="M14 2v6h6"/>
								<path d="M2 15h10"/>
								<path d="M2 19h10"/>
								<path d="M5 12v10"/>
							</svg>
							<span>Modo SQL</span>
						</button>
					{:else}
						<div class="sql-status-bar">
							<span class="sql-badge">
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<circle cx="12" cy="12" r="10"/>
									<polyline points="12 6 12 12 16 14"/>
								</svg>
								Executado em {fileStore.sqlExecutionTime}ms
							</span>
							<button class="btn-exit-sql" onclick={() => fileStore.exitSqlMode()}>
								Encerrar Modo SQL
							</button>
						</div>
					{/if}
				</div>
				<div class="metadata-wrapper">
					<MetadataPanel 
						sourcePath={fileStore.sourcePath ?? ''} 
						totalRows={fileStore.totalRows} 
						schema={fileStore.schema} 
						dataSource={fileStore.dataSource}
						files={fileStore.files}
					/>
				</div>
			</div>

			<div class="flex-table-container">
				<DataTable
					schema={fileStore.schema}
					rows={fileStore.rows}
					startRowIndex={fileStore.currentPage * fileStore.pageSize}
					sortCol={fileStore.sort.column}
					sortOrder={fileStore.sort.order}
					onsort={fileStore.handleSort}
				/>
			</div>

			<div class="footer-actions">
				<Pagination
					currentPage={fileStore.currentPage}
					totalPages={fileStore.totalPages}
					isLoading={fileStore.isLoading}
					rowsLength={fileStore.rows.length}
					pageSize={fileStore.pageSize}
					onprev={fileStore.prevPage}
					onnext={fileStore.nextPage}
				/>
			</div>
		{/if}
	</div>

	{#if uiStore.showAbout}
		<Modal title={`Sobre o ${APP_INFO.name}`} onclose={() => uiStore.closeAbout()}>
			<div class="about-content">
				<img src="/welcome.svg" alt="Logo" style="height: 80px; margin-bottom: 1rem;" />
				<p style="font-size: 1.2rem; font-weight: 700; margin-bottom: 0.2rem;">
					{APP_INFO.name}
					<span style="font-weight:400; opacity: 0.7;">{APP_INFO.version}</span>
				</p>
				<a 
					href={APP_INFO.social.githubRepo} 
					class="repo-link" 
					onclick={(e) => handleExternalClick(e, APP_INFO.social.githubRepo)}
				>
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"/>
					</svg>
					Ver código fonte no GitHub
				</a>
				<div class="divider"></div>
				<p class="dev-info">
					Desenvolvido com 💚 por <strong>{APP_INFO.author.name}</strong><br />
					<span class="tech-stack">{APP_INFO.author.stack}</span>
				</p>
				<div class="social-links">
					<a href={APP_INFO.social.githubProfile} class="social-btn github" aria-label="GitHub" onclick={(e) => handleExternalClick(e, APP_INFO.social.githubProfile)}>
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"/>
						</svg>
					</a>
					<a href={APP_INFO.social.linkedin} class="social-btn linkedin" aria-label="LinkedIn" onclick={(e) => handleExternalClick(e, APP_INFO.social.linkedin)}>
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"/>
							<rect x="2" y="9" width="4" height="12"/>
							<circle cx="4" cy="4" r="2"/>
						</svg>
					</a>
					<a href={APP_INFO.social.email} class="social-btn email" aria-label="Email" onclick={(e) => handleExternalClick(e, APP_INFO.social.email)}>
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/>
							<polyline points="22,6 12,13 2,6"/>
						</svg>
					</a>
				</div>
				<a href={APP_INFO.attribution.storyset.url} class="attribution" onclick={(e) => handleExternalClick(e, APP_INFO.attribution.storyset.url)}>
					{APP_INFO.attribution.storyset.text}
				</a>
			</div>
		</Modal>
	{/if}

	<SqlModal 
		isOpen={uiStore.showSql} 
		onclose={() => uiStore.closeSql()} 
		onrun={handleRunSql} 
	/>
	
	<ExportModal 
		isOpen={uiStore.showExport} 
		isSqlMode={fileStore.isSqlMode} 
		isLoading={uiStore.isExporting} 
		onclose={() => uiStore.closeExport()} 
		onexport={handleExport} 
	/>
</main>

<style src="./page.css"></style>

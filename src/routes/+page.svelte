<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { open } from "@tauri-apps/plugin-dialog";
	import { listen } from "@tauri-apps/api/event";
	import { onMount, onDestroy } from "svelte";
	import { APP_INFO } from "$lib/constants";
	import { openUrl } from "@tauri-apps/plugin-opener";

	import Titlebar from "./components/Titlebar/Titlebar.svelte";
	import Pagination from "./components/Pagination/Pagination.svelte";
	import DataTable, {
		type ColumnInfo,
		type DataRow,
	} from "./components/DataTable/DataTable.svelte";
	import MetadataPanel from "./components/MetadataPanel/MetadataPanel.svelte";
	import Modal from "./components/Modal/Modal.svelte";

	let isDark = true;
	let showAboutModal = false;
	let isDragging = false;
	let unlistenDrop: () => void;
	let unlistenHover: () => void;
	let unlistenCancel: () => void;

	$: if (typeof document !== "undefined") {
		document.body.classList.toggle("dark-mode", isDark);
	}

	function toggleTheme() {
		isDark = !isDark;
	}

	interface FileMetadata {
		file_path: string;
		total_rows: number;
		schema: ColumnInfo[];
	}

	let schema: ColumnInfo[] = [];
	let rows: DataRow[] = [];
	let errorMsg = "";
	let isLoading = false;
	let selectedFile = "";
	let totalRows = 0;
	let currentPage = 0;
	const pageSize = 50;

	$: totalPages = Math.ceil(totalRows / pageSize);
	$: hasData = schema.length > 0;

	onMount(async () => {
		unlistenDrop = await listen("tauri://drag-drop", (event: any) => {
			isDragging = false;
			if (event.payload.paths && event.payload.paths.length > 0) {
				const path = event.payload.paths[0];
				if (path.endsWith(".parquet")) {
					loadParquetFile(path);
				} else {
					errorMsg = "Por favor, solte apenas arquivos .parquet";
				}
			}
		});

		unlistenHover = await listen("tauri://drag-enter", () => {
			isDragging = true;
		});

		unlistenCancel = await listen("tauri://drag-leave", () => {
			isDragging = false;
		});
	});

	onDestroy(() => {
		if (unlistenDrop) unlistenDrop();
		if (unlistenHover) unlistenHover();
		if (unlistenCancel) unlistenCancel();
	});

	async function loadParquetFile(filePath: string) {
		isLoading = true;
		errorMsg = "";

		// Reset limpo
		schema = [];
		rows = [];
		selectedFile = filePath;
		currentPage = 0;
		totalRows = 0;

		try {
			const metadata: FileMetadata = await invoke("get_file_metadata", {
				filePath: filePath,
			});
			schema = metadata.schema;
			totalRows = metadata.total_rows;

			if (schema.length > 0) {
				await loadPage(0);
			}
		} catch (e) {
			console.error("Erro ao carregar arquivo:", e);
			errorMsg = e as string;
		} finally {
			isLoading = false;
		}
	}

	async function handleOpenFile() {
		try {
			const file = await open({
				title: "Selecione um arquivo Parquet",
				multiple: false,
				filters: [{ name: "Parquet", extensions: ["parquet"] }],
			});

			if (file && typeof file === "string") {
				await loadParquetFile(file);
			}
		} catch (e) {
			errorMsg = e as string;
		}
	}

	async function loadPage(page: number) {
		if (!selectedFile) return;
		isLoading = true;
		errorMsg = "";

		try {
			rows = await invoke("get_page_data", {
				filePath: selectedFile,
				page: page,
				pageSize: pageSize,
			});
			currentPage = page;
		} catch (e) {
			console.error("Erro ao carregar página:", e);
			errorMsg = e as string;
		} finally {
			isLoading = false;
		}
	}

	async function openExternal(url: string) {
		try {
			await openUrl(url);
		} catch (e) {
			console.error("Falha ao abrir link:", e);
		}
	}
</script>

<main class="container">
	<Titlebar
		onopenfile={handleOpenFile}
		{isDark}
		ontoggle={toggleTheme}
		onabout={() => (showAboutModal = true)}
	/>

	<div class="content">
		{#if isDragging}
			<div class="drop-overlay">
				<div class="drop-message">
					<svg
						width="48"
						height="48"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						><path
							d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"
						/><polyline points="17 8 12 3 7 8" /><line
							x1="12"
							y1="3"
							x2="12"
							y2="15"
						/></svg
					>
					<span>Solte o arquivo Parquet aqui</span>
				</div>
			</div>
		{/if}

		{#if !hasData && !errorMsg}
			<div class="empty-state-container">
				<img
					src="/welcome.svg"
					alt="Bem-vindo ao OpenParquet"
					class="welcome-image"
				/>

				<h2 class="welcome-title">{APP_INFO.name}</h2>
				<p class="welcome-subtitle">
					Arraste um arquivo ou clique abaixo para começar.
				</p>

				<button
					class="btn-primary large-btn"
					on:click={handleOpenFile}
					disabled={isLoading}
				>
					{#if isLoading}
						<span class="loader"></span> Carregando...
					{:else}
						<svg
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							style="margin-right:8px"
							><path
								d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"
							/><polyline points="17 8 12 3 7 8" /><line
								x1="12"
								y1="3"
								x2="12"
								y2="15"
							/></svg
						>
						Abrir Arquivo Parquet
					{/if}
				</button>
			</div>
		{/if}

		{#if errorMsg}
			<div class="actions-bar">
				<div class="error-banner">
					<strong>Erro:</strong>
					{errorMsg}
				</div>
				<button class="btn-primary" on:click={handleOpenFile}
					>Tentar Outro Arquivo</button
				>
			</div>
		{/if}

		{#if hasData}
			<MetadataPanel filePath={selectedFile} {totalRows} {schema} />

			<div class="flex-table-container">
				<DataTable
					{schema}
					{rows}
					startRowIndex={currentPage * pageSize}
				/>
			</div>

			<div class="footer-actions">
				<Pagination
					{currentPage}
					{totalPages}
					{isLoading}
					rowsLength={rows.length}
					{pageSize}
					onprev={() => loadPage(currentPage - 1)}
					onnext={() => loadPage(currentPage + 1)}
				/>
			</div>
		{/if}
	</div>

	{#if showAboutModal}
		<Modal
			title={`Sobre o ${APP_INFO.name}`}
			onclose={() => (showAboutModal = false)}
		>
			<div class="about-content">
				<img
					src="/welcome.svg"
					alt="Logo"
					style="height: 80px; margin-bottom: 1rem;"
				/>

				<p
					style="font-size: 1.2rem; font-weight: 700; margin-bottom: 0.2rem;"
				>
					{APP_INFO.name}
					<span style="font-weight:400; opacity: 0.7;"
						>{APP_INFO.version}</span
					>
				</p>

				<a
					href={APP_INFO.social.githubRepo}
					class="repo-link"
					on:click|preventDefault={() =>
						openExternal(APP_INFO.social.githubRepo)}
				>
					<svg
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						><path
							d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"
						></path></svg
					>
					Ver código fonte no GitHub
				</a>

				<div class="divider"></div>

				<p class="dev-info">
					Desenvolvido com 💚 por <strong
						>{APP_INFO.author.name}</strong
					><br />
					<span class="tech-stack">{APP_INFO.author.stack}</span>
				</p>

				<div class="social-links">
					<a
						href={APP_INFO.social.githubProfile}
						class="social-btn github"
						aria-label="Perfil no GitHub"
						on:click|preventDefault={() =>
							openExternal(APP_INFO.social.githubProfile)}
					>
						<svg
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							><path
								d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"
							></path></svg
						>
					</a>

					<a
						href={APP_INFO.social.linkedin}
						class="social-btn linkedin"
						aria-label="Perfil no LinkedIn"
						on:click|preventDefault={() =>
							openExternal(APP_INFO.social.linkedin)}
					>
						<svg
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							><path
								d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"
							/><rect x="2" y="9" width="4" height="12" /><circle
								cx="4"
								cy="4"
								r="2"
							/></svg
						>
					</a>

					<a
						href={APP_INFO.social.email}
						class="social-btn email"
						aria-label="Enviar Email"
						on:click|preventDefault={() =>
							openExternal(APP_INFO.social.email)}
					>
						<svg
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							><path
								d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"
							/><polyline points="22,6 12,13 2,6" /></svg
						>
					</a>
				</div>

				<a
					href={APP_INFO.attribution.storyset.url}
					class="attribution"
					on:click|preventDefault={() =>
						openExternal(APP_INFO.attribution.storyset.url)}
				>
					{APP_INFO.attribution.storyset.text}
				</a>
			</div>
		</Modal>
	{/if}
</main>

<style src="./page.css"></style>

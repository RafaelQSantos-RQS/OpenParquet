import { invoke } from '@tauri-apps/api/core';
import { getRecentFiles, addRecentFile as addToRecent } from '$lib/preferences';
import type { 
	ColumnInfo, 
	DataRow, 
	FileMetadata, 
	MultiFileMetadata,
	FileListMetadata,
	ParquetFileInfo,
	QueryResult, 
	SortState,
	DataSource 
} from '$lib/types';

const PAGE_SIZE = 50;

let dataSource = $state<DataSource>('file');
let filePath = $state<string | null>(null);
let directoryPath = $state<string | null>(null);
let fileList = $state<string[]>([]);
let files = $state<ParquetFileInfo[]>([]);
let schema = $state<ColumnInfo[]>([]);
let rows = $state<DataRow[]>([]);
let totalRows = $state(0);
let currentPage = $state(0);
let isLoading = $state(false);
let error = $state<string | null>(null);

let sort = $state<SortState>({ column: null, order: null });

let isSqlMode = $state(false);
let currentSqlQuery = $state('');
let sqlExecutionTime = $state(0);

let recentFiles = $state<string[]>([]);

let hasData = $derived(schema.length > 0);
let totalPages = $derived(Math.ceil(totalRows / PAGE_SIZE));
let isMultiFileMode = $derived((dataSource === 'directory' || dataSource === 'fileList') && files.length > 0);
let sourcePath = $derived(dataSource === 'file' ? filePath : dataSource === 'directory' ? directoryPath : null);
let filePaths = $derived(dataSource === 'fileList' ? fileList : null);

async function loadRecentFiles(): Promise<void> {
	recentFiles = await getRecentFiles();
}

async function loadFile(path: string): Promise<void> {
	dataSource = 'file';
	isLoading = true;
	error = null;
	
	resetData();
	filePath = path;
	
	try {
		const metadata = await invoke<FileMetadata>('get_file_metadata', { filePath: path });
		schema = metadata.schema;
		totalRows = metadata.total_rows;
		
		await addToRecent(path);
		await loadRecentFiles();
		
		if (schema.length > 0) {
			await loadPage(0);
		}
	} catch (e) {
		console.error('Erro ao carregar arquivo:', e);
		error = e as string;
	} finally {
		isLoading = false;
	}
}

async function loadDirectory(path: string): Promise<void> {
	dataSource = 'directory';
	isLoading = true;
	error = null;
	
	resetData();
	directoryPath = path;
	
	try {
		const metadata = await invoke<MultiFileMetadata>('get_multi_file_metadata', { directoryPath: path });
		schema = metadata.schema;
		totalRows = metadata.total_rows;
		files = metadata.files;
		
		await addToRecent(path);
		await loadRecentFiles();
		
		if (schema.length > 0 && files.length > 0) {
			await loadPage(0);
		}
	} catch (e) {
		console.error('Erro ao carregar pasta:', e);
		error = e as string;
	} finally {
		isLoading = false;
	}
}

async function loadFileList(paths: string[]): Promise<void> {
	if (paths.length === 0) return;
	
	dataSource = 'fileList';
	isLoading = true;
	error = null;
	
	resetData();
	fileList = paths;
	
	try {
		const metadata = await invoke<FileListMetadata>('get_file_list_metadata', { filePaths: paths });
		schema = metadata.schema;
		totalRows = metadata.total_rows;
		files = metadata.files;
		
		const key = paths.join('|');
		await addToRecent(key);
		await loadRecentFiles();
		
		if (schema.length > 0 && files.length > 0) {
			await loadPage(0);
		}
	} catch (e) {
		console.error('Erro ao carregar lista de arquivos:', e);
		error = e as string;
	} finally {
		isLoading = false;
	}
}

async function loadPage(page: number): Promise<void> {
	isLoading = true;
	error = null;
	
	try {
		if (isSqlMode) {
			const result = await invoke<QueryResult>('run_sql', {
				sourcePath: sourcePath ?? '',
				query: currentSqlQuery,
				page,
				pageSize: PAGE_SIZE,
				filePaths
			});
			rows = result.rows;
		} else if (dataSource === 'fileList' && fileList.length > 0) {
			let query = 'SELECT * FROM t';
			if (sort.column && sort.order) {
				query += ` ORDER BY "${sort.column}" ${sort.order}`;
			}
			const result = await invoke<QueryResult>('run_sql', {
				sourcePath: '',
				query,
				page,
				pageSize: PAGE_SIZE,
				filePaths: fileList
			});
			rows = result.rows;
		} else if (dataSource === 'directory' && directoryPath) {
			rows = await invoke<DataRow[]>('get_multi_file_page_data', {
				directoryPath,
				page,
				pageSize: PAGE_SIZE,
				sortCol: sort.column,
				sortOrder: sort.order
			});
		} else if (dataSource === 'file' && filePath) {
			rows = await invoke<DataRow[]>('get_page_data', {
				filePath,
				page,
				pageSize: PAGE_SIZE,
				sortCol: sort.column,
				sortOrder: sort.order
			});
		}
		currentPage = page;
	} catch (e) {
		console.error('Erro ao carregar página:', e);
		error = e as string;
	} finally {
		isLoading = false;
	}
}

function handleSort(columnName: string): void {
	if (isSqlMode) return;
	
	if (sort.column === columnName) {
		sort.order = sort.order === 'ASC' ? 'DESC' : 'ASC';
	} else {
		sort.column = columnName;
		sort.order = 'ASC';
	}
	loadPage(currentPage);
}

async function runSql(query: string): Promise<void> {
	isLoading = true;
	error = null;
	currentSqlQuery = query;
	
	try {
		const result = await invoke<QueryResult>('run_sql', {
			sourcePath: sourcePath ?? '',
			query,
			page: 0,
			pageSize: PAGE_SIZE,
			filePaths
		});
		
		schema = result.schema;
		rows = result.rows;
		totalRows = result.total_rows;
		sqlExecutionTime = result.execution_time_ms;
		
		isSqlMode = true;
		currentPage = 0;
		sort = { column: null, order: null };
	} catch (e) {
		console.error('Erro na query:', e);
		error = `Erro na query: ${e}`;
	} finally {
		isLoading = false;
	}
}

function exitSqlMode(): void {
	isSqlMode = false;
	currentSqlQuery = '';
	
	if (dataSource === 'fileList' && fileList.length > 0) {
		loadFileList(fileList);
	} else if (dataSource === 'directory' && directoryPath) {
		loadDirectory(directoryPath);
	} else if (dataSource === 'file' && filePath) {
		loadFile(filePath);
	}
}

async function nextPage(): Promise<void> {
	if (currentPage < totalPages - 1) {
		await loadPage(currentPage + 1);
	}
}

async function prevPage(): Promise<void> {
	if (currentPage > 0) {
		await loadPage(currentPage - 1);
	}
}

function resetData(): void {
	schema = [];
	rows = [];
	totalRows = 0;
	currentPage = 0;
	sort = { column: null, order: null };
	isSqlMode = false;
	currentSqlQuery = '';
	sqlExecutionTime = 0;
	files = [];
}

export const fileStore = {
	get dataSource(): DataSource {
		return dataSource;
	},
	get filePath(): string | null {
		return filePath;
	},
	get directoryPath(): string | null {
		return directoryPath;
	},
	get fileList(): string[] {
		return fileList;
	},
	get files(): ParquetFileInfo[] {
		return files;
	},
	get schema(): ColumnInfo[] {
		return schema;
	},
	get rows(): DataRow[] {
		return rows;
	},
	get totalRows(): number {
		return totalRows;
	},
	get currentPage(): number {
		return currentPage;
	},
	get totalPages(): number {
		return totalPages;
	},
	get isLoading(): boolean {
		return isLoading;
	},
	get error(): string | null {
		return error;
	},
	get sort(): SortState {
		return sort;
	},
	get isSqlMode(): boolean {
		return isSqlMode;
	},
	get currentSqlQuery(): string {
		return currentSqlQuery;
	},
	get sqlExecutionTime(): number {
		return sqlExecutionTime;
	},
	get recentFiles(): string[] {
		return recentFiles;
	},
	get hasData(): boolean {
		return hasData;
	},
	get pageSize(): number {
		return PAGE_SIZE;
	},
	get isMultiFileMode(): boolean {
		return isMultiFileMode;
	},
	get sourcePath(): string | null {
		return sourcePath;
	},
	get filePaths(): string[] | null {
		return filePaths;
	},
	
	loadFile,
	loadDirectory,
	loadFileList,
	loadPage,
	loadRecentFiles,
	handleSort,
	runSql,
	exitSqlMode,
	nextPage,
	prevPage
};

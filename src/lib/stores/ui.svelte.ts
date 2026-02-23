let showAbout = $state(false);
let showSql = $state(false);
let showExport = $state(false);
let isExporting = $state(false);
let isDragging = $state(false);

function openAbout(): void {
	showAbout = true;
}

function closeAbout(): void {
	showAbout = false;
}

function openSql(): void {
	showSql = true;
}

function closeSql(): void {
	showSql = false;
}

function openExport(): void {
	showExport = true;
}

function closeExport(): void {
	showExport = false;
}

function setDragging(value: boolean): void {
	isDragging = value;
}

function setExporting(value: boolean): void {
	isExporting = value;
}

export const uiStore = {
	get showAbout(): boolean {
		return showAbout;
	},
	get showSql(): boolean {
		return showSql;
	},
	get showExport(): boolean {
		return showExport;
	},
	get isExporting(): boolean {
		return isExporting;
	},
	get isDragging(): boolean {
		return isDragging;
	},
	
	openAbout,
	closeAbout,
	openSql,
	closeSql,
	openExport,
	closeExport,
	setDragging,
	setExporting
};

import { LazyStore } from "@tauri-apps/plugin-store";

let store: LazyStore | null = null;

function getStore(): LazyStore {
	if (!store) {
		store = new LazyStore('preferences.json');
	}
	return store;
}

export async function getTheme(): Promise<'dark' | 'light' | null> {
    try {
        const val = await getStore().get('theme');
        return val as 'dark' | 'light' | null;
    } catch (e) {
        console.error('Erro ao ler tema:', e);
        return null;
    }
}

export async function setTheme(theme: 'dark' | 'light') {
    try {
        await getStore().set('theme', theme);
        await getStore().save();
    } catch (e) {
        console.error('Erro ao salvar tema:', e);
    }
}

export async function getRecentFiles(): Promise<string[]> {
    try {
        const val = await getStore().get('recent_files');
        return (val as string[]) || [];
    } catch (e) {
        return [];
    }
}

export async function addRecentFile(filePath: string) {
    try {
        let files = (await getStore().get('recent_files') as string[]) || [];
        
        files = files.filter(f => f !== filePath);
        files.unshift(filePath);
        
        if (files.length > 10) files = files.slice(0, 10);
        
        await getStore().set('recent_files', files);
        await getStore().save();
    } catch (e) {
        console.error('Erro ao salvar recente:', e);
    }
}
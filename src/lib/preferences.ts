import { LazyStore } from "@tauri-apps/plugin-store";

const store = new LazyStore('preferences.json');

export async function getTheme(): Promise<'dark' | 'light' | null> {
    try {
        const val = await store.get('theme');
        return val as 'dark' | 'light' | null;
    } catch (e) {
        console.error('Erro ao ler tema:', e);
        return null;
    }
}

export async function setTheme(theme: 'dark' | 'light') {
    try {
        await store.set('theme', theme);
        await store.save();
    } catch (e) {
        console.error('Erro ao salvar tema:', e);
    }
}

export async function getRecentFiles(): Promise<string[]> {
    try {
        const val = await store.get('recent_files');
        return (val as string[]) || [];
    } catch (e) {
        return [];
    }
}

export async function addRecentFile(filePath: string) {
    try {
        let files = (await store.get('recent_files') as string[]) || [];
        
        files = files.filter(f => f !== filePath);
        files.unshift(filePath);
        
        if (files.length > 10) files = files.slice(0, 10);
        
        await store.set('recent_files', files);
        await store.save();
    } catch (e) {
        console.error('Erro ao salvar recente:', e);
    }
}
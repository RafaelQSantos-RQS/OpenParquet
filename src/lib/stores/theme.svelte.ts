import { getTheme, setTheme as persistTheme } from '$lib/preferences';
import { onMount } from 'svelte';
import type { Theme } from '$lib/types';

let isDark = $state(true);
let initialized = $state(false);

async function init(): Promise<void> {
	if (initialized) return;
	
	const saved = await getTheme();
	if (saved) {
		isDark = saved === 'dark';
	}
	updateBodyClass();
	initialized = true;
}

function toggle(): void {
	isDark = !isDark;
	persistTheme(isDark ? 'dark' : 'light');
	updateBodyClass();
}

function setTheme(theme: Theme): void {
	const shouldToggle = (theme === 'dark' && !isDark) || (theme === 'light' && isDark);
	if (shouldToggle) {
		toggle();
	}
}

function updateBodyClass(): void {
	if (typeof document !== 'undefined') {
		document.body.classList.toggle('dark-mode', isDark);
	}
}

export const themeStore = {
	get isDark(): boolean {
		return isDark;
	},
	get theme(): Theme {
		return isDark ? 'dark' : 'light';
	},
	init,
	toggle,
	setTheme
};

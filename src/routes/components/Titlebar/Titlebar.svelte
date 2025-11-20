<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount, onDestroy } from 'svelte';

	const appWindow = getCurrentWindow();

	export let isDark: boolean;
    
    // NOVO: Definindo as funções de callback (Substitui createEventDispatcher)
    export let onopenfile: () => void = () => {};
    export let ontoggle: () => void = () => {};
    export let onabout: () => void = () => {};

	let isMaximized = false;
	let unlisten: Function | null = null;

	let openMenu: 'file' | 'theme' | 'help' | null = null;

	function handleMenuClick(menu: 'file' | 'theme' | 'help') {
		openMenu = openMenu === menu ? null : menu;
	}

	function handleFileOptionClick(action: 'open') {
		openMenu = null;
		if (action === 'open') {
			onopenfile(); // Chama direto
		}
	}

	function handleThemeSelect(mode: 'light' | 'dark') {
		openMenu = null;
		const shouldToggle = (mode === 'dark' && !isDark) || (mode === 'light' && isDark);
		if (shouldToggle) {
			ontoggle(); // Chama direto
		}
	}

	function handleHelpOptionClick(action: 'about') {
		openMenu = null;
		if (action === 'about') {
			onabout(); // Chama direto
		}
	}

	function clickOutside(node: HTMLElement) {
		const handleClick = (event: MouseEvent) => {
			if (openMenu && node && !node.contains(event.target as Node)) {
				openMenu = null;
			}
		};
		document.addEventListener('click', handleClick, true);
		return {
			destroy() {
				document.removeEventListener('click', handleClick, true);
			}
		};
	}

	onMount(async () => {
		isMaximized = await appWindow.isMaximized();
		unlisten = await appWindow.onResized(async () => {
			isMaximized = await appWindow.isMaximized();
		});
	});
	
	onDestroy(() => {
		if (unlisten) unlisten();
	});
</script>

<div data-tauri-drag-region class="titlebar" class:maximized={isMaximized}>
	<img 
        src="/app-icon.png" 
        alt="App Icon" 
        class="window-icon" 
        draggable="false" 
    />
	<div class="titlebar-menu-container" use:clickOutside>
		<div class="titlebar-menu">
			<button type="button" class="menu-button" on:click={() => handleMenuClick('file')} class:open={openMenu === 'file'}>Arquivo</button>
			<button type="button" class="menu-button" on:click={() => handleMenuClick('theme')} class:open={openMenu === 'theme'}>Tema</button>
			<button type="button" class="menu-button" on:click={() => handleMenuClick('help')} class:open={openMenu === 'help'}>Ajuda</button>
		</div>

		{#if openMenu === 'file'}
			<div class="menu-dropdown file-menu">
				<button type="button" class="menu-item" on:click={() => handleFileOptionClick('open')}>Abrir...</button>
				<button type="button" class="menu-item" on:click={() => appWindow.close()}>Sair</button>
			</div>
		{/if}
		
		{#if openMenu === 'theme'}
			<div class="menu-dropdown theme-menu">
				<button type="button" class="menu-item" class:active={!isDark} on:click={() => handleThemeSelect('light')}>Claro</button>
				<button type="button" class="menu-item" class:active={isDark} on:click={() => handleThemeSelect('dark')}>Escuro</button>
			</div>
		{/if}

		{#if openMenu === 'help'}
			<div class="menu-dropdown help-menu" style="left: 130px;">
				<button type="button" class="menu-item" on:click={() => handleHelpOptionClick('about')}>Sobre</button>
			</div>
		{/if}
	</div>

	<h1 class="titlebar-title">OpenParquet</h1>

	<div class="titlebar-controls">
		<button type="button" class="titlebar-button" on:click={() => appWindow.minimize()} aria-label="Minimizar">
			<svg width="11" height="11" viewBox="0 0 12 12"><path d="M0 5h12v2H0z" /></svg>
		</button>
		<button type="button" class="titlebar-button" on:click={() => appWindow.toggleMaximize()} aria-label="Maximizar">
			<svg width="11" height="11" viewBox="0 0 12 12"><path d="M2 2h8v8H2zM0 0v12h12V0H0z" /></svg>
		</button>
		<button type="button" class="titlebar-button close" on:click={() => appWindow.close()} aria-label="Fechar">
			<svg width="11" height="11" viewBox="0 0 12 12"><path d="M10.2 0L6 4.2 1.8 0 0 1.8 4.2 6 0 10.2 1.8 12 6 7.8 10.2 12 12 10.2 7.8 6 12 1.8z"/></svg>
		</button>
	</div>
</div>

<style src="./Titlebar.css"></style>
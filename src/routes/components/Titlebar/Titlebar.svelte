<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount, onDestroy } from 'svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { uiStore } from '$lib/stores/ui.svelte';
	import { clickOutside } from '$lib/actions/click-outside';

	interface Props {
		onopenfile?: () => void;
		onopenfiles?: () => void;
		onopendirectory?: () => void;
	}

	let { onopenfile = () => {}, onopenfiles = () => {}, onopendirectory = () => {} }: Props = $props();

	let appWindow = $state<ReturnType<typeof getCurrentWindow> | null>(null);
	let openMenu = $state<'file' | 'theme' | 'help' | null>(null);
	let isMaximized = $state(false);
	let unlisten: (() => void) | null = null;

	function handleMenuClick(menu: 'file' | 'theme' | 'help'): void {
		openMenu = openMenu === menu ? null : menu;
	}

	function handleFileOption(action: 'open' | 'open-files' | 'open-directory'): void {
		openMenu = null;
		if (action === 'open') {
			onopenfile();
		} else if (action === 'open-files') {
			onopenfiles();
		} else if (action === 'open-directory') {
			onopendirectory();
		}
	}

	function handleHelpOption(action: 'about'): void {
		openMenu = null;
		uiStore.openAbout();
	}

	function closeMenu(): void {
		openMenu = null;
	}

	function minimize(): void {
		if (appWindow) appWindow.minimize();
	}

	function toggleMaximize(): void {
		if (appWindow) appWindow.toggleMaximize();
	}

	function close(): void {
		if (appWindow) appWindow.close();
	}

	onMount(async () => {
		appWindow = getCurrentWindow();
		isMaximized = await appWindow.isMaximized();
		unlisten = await appWindow.onResized(async () => {
			if (appWindow) {
				isMaximized = await appWindow.isMaximized();
			}
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
	
	<div class="titlebar-menu-container" use:clickOutside={closeMenu}>
		<div class="titlebar-menu">
			<button 
				type="button" 
				class="menu-button" 
				class:open={openMenu === 'file'}
				onclick={() => handleMenuClick('file')}
			>
				Arquivo
			</button>
			<button 
				type="button" 
				class="menu-button" 
				class:open={openMenu === 'theme'}
				onclick={() => handleMenuClick('theme')}
			>
				Tema
			</button>
			<button 
				type="button" 
				class="menu-button" 
				class:open={openMenu === 'help'}
				onclick={() => handleMenuClick('help')}
			>
				Ajuda
			</button>
		</div>

		{#if openMenu === 'file'}
			<div class="menu-dropdown file-menu">
				<button type="button" class="menu-item" onclick={() => handleFileOption('open')}>
					Abrir Arquivo...
				</button>
				<button type="button" class="menu-item" onclick={() => handleFileOption('open-files')}>
					Abrir Múltiplos Arquivos...
				</button>
				<button type="button" class="menu-item" onclick={() => handleFileOption('open-directory')}>
					Abrir Pasta...
				</button>
				<div class="menu-divider"></div>
				<button type="button" class="menu-item" onclick={close}>
					Sair
				</button>
			</div>
		{/if}
		
		{#if openMenu === 'theme'}
			<div class="menu-dropdown theme-menu">
				<button 
					type="button" 
					class="menu-item" 
					class:active={!themeStore.isDark}
					onclick={() => { themeStore.setTheme('light'); closeMenu(); }}
				>
					Claro
				</button>
				<button 
					type="button" 
					class="menu-item" 
					class:active={themeStore.isDark}
					onclick={() => { themeStore.setTheme('dark'); closeMenu(); }}
				>
					Escuro
				</button>
			</div>
		{/if}

		{#if openMenu === 'help'}
			<div class="menu-dropdown help-menu" style="left: 130px;">
				<button type="button" class="menu-item" onclick={() => handleHelpOption('about')}>
					Sobre
				</button>
			</div>
		{/if}
	</div>

	<h1 class="titlebar-title">OpenParquet</h1>

	<div class="titlebar-controls">
		<button 
			type="button" 
			class="titlebar-button" 
			onclick={minimize} 
			aria-label="Minimizar"
		>
			<svg width="11" height="11" viewBox="0 0 12 12"><path d="M0 5h12v2H0z" /></svg>
		</button>
		<button 
			type="button" 
			class="titlebar-button" 
			onclick={toggleMaximize} 
			aria-label="Maximizar"
		>
			<svg width="11" height="11" viewBox="0 0 12 12"><path d="M2 2h8v8H2zM0 0v12h12V0H0z" /></svg>
		</button>
		<button 
			type="button" 
			class="titlebar-button close" 
			onclick={close} 
			aria-label="Fechar"
		>
			<svg width="11" height="11" viewBox="0 0 12 12"><path d="M10.2 0L6 4.2 1.8 0 0 1.8 4.2 6 0 10.2 1.8 12 6 7.8 10.2 12 12 10.2 7.8 6 12 1.8z"/></svg>
		</button>
	</div>
</div>

<style src="./Titlebar.css"></style>

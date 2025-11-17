<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount, onDestroy, createEventDispatcher } from 'svelte';

	const appWindow = getCurrentWindow();
	const dispatch = createEventDispatcher();

	// PROPS:
	// isDark é o estado atual do tema, vindo do +page.svelte
	export let isDark: boolean;

	let isMaximized = false;
	let unlisten: Function | null = null;

	// --- Lógica do Menu ---

	// AGORA: openMenu rastreia qual menu está aberto ('file', 'theme', ou null)
	let openMenu: 'file' | 'theme' | null = null;

	function handleMenuClick(menu: 'file' | 'theme') {
		// Abre o menu, ou fecha se for clicado novamente
		openMenu = openMenu === menu ? null : menu;
	}

	function handleFileOptionClick(action: 'open') {
		openMenu = null; // Fecha o menu ao clicar
		if (action === 'open') {
			dispatch('openfile'); // Dispara o evento 'openfile' para o pai
		}
	}

	function handleThemeSelect(mode: 'light' | 'dark') {
		openMenu = null; // Fecha o menu
		
		// Determina se uma alternância é necessária. 
		// Só alterna se o tema selecionado for diferente do tema atual.
		const shouldToggle = (mode === 'dark' && !isDark) || (mode === 'light' && isDark);

		if (shouldToggle) {
			dispatch('toggle'); // Dispara o evento 'toggle' para o pai
		}
	}

	// Action 'clickOutside': fecha o menu se o clique for fora do contêiner.
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
	// --- Fim da Lógica do Menu ---

	onMount(async () => {
		isMaximized = await appWindow.isMaximized();
		unlisten = await appWindow.onResized(async () => {
			isMaximized = await appWindow.isMaximized();
		});
	});

	onDestroy(() => {
		if (unlisten) {
			unlisten();
		}
	});
</script>

<div data-tauri-drag-region class="titlebar" class:maximized={isMaximized}>
	<div class="titlebar-menu-container" use:clickOutside>
		<div class="titlebar-menu">
			
			<button 
				type="button" 
				class="menu-button" 
				on:click={() => handleMenuClick('file')}
				class:open={openMenu === 'file'}
			> 
				Arquivo 
			</button>

			<button 
				type="button" 
				class="menu-button" 
				on:click={() => handleMenuClick('theme')}
				class:open={openMenu === 'theme'}
			> 
				Tema 
			</button>
		</div>

		{#if openMenu === 'file'}
			<div class="menu-dropdown file-menu">
				<button
					type="button"
					class="menu-item"
					on:click={() => handleFileOptionClick('open')}
				>
					Abrir o arquivo...
				</button>
				<button
					type="button"
					class="menu-item"
					on:click={() => appWindow.close()}
				>
					Sair
				</button>
			</div>
		{/if}
		
		{#if openMenu === 'theme'}
			<div class="menu-dropdown theme-menu">
				<button
					type="button"
					class="menu-item"
					class:active={!isDark}
					on:click={() => handleThemeSelect('light')}
				>
					Claro
				</button>
				<button
					type="button"
					class="menu-item"
					class:active={isDark}
					on:click={() => handleThemeSelect('dark')}
				>
					Escuro
				</button>
			</div>
		{/if}
	</div>

	<h1 class="titlebar-title">OpenParquet</h1>

	<div class="titlebar-controls">
		<button
			type="button"
			class="titlebar-button"
			on:click={() => appWindow.minimize()}
			aria-label="Minimizar"
		>
			<svg width="12" height="12" viewBox="0 0 12 12"><path d="M0 5h12v2H0z" fill="var(--color-subtle-text)" /></svg>
		</button>
		<button
			type="button"
			class="titlebar-button"
			on:click={() => appWindow.toggleMaximize()}
			aria-label="Maximizar"
		>
			<svg width="12" height="12" viewBox="0 0 12 12"
				><path d="M2 2h8v8H2zM0 0v12h12V0H0z" fill="var(--color-subtle-text)" /></svg
			>
		</button>
		<button
			type="button"
			class="titlebar-button close"
			on:click={() => appWindow.close()}
			aria-label="Fechar"
		>
			<svg width="12" height="12" viewBox="0 0 12 12"
				><path
					d="M10.2 0L6 4.2 1.8 0 0 1.8 4.2 6 0 10.2 1.8 12 6 7.8 10.2 12 12 10.2 7.8 6 12 1.8z"
					fill="var(--color-subtle-text)"
				/></svg
			>
		</button>
	</div>
</div>

<style>
	.titlebar {
		height: 30px;
		background: var(--color-titlebar-bg);
		display: flex;
		justify-content: space-between;
		align-items: center;
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
		position: relative;
		user-select: none;
	}
	.titlebar.maximized {
		border-top-left-radius: 0;
		border-top-right-radius: 0;
	}
	.titlebar-title {
		position: absolute;
		left: 0;
		right: 0;
		text-align: center;
		margin: 0;
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text);
		pointer-events: none;
	}

	/* --- Menu (Esquerda) --- */

	.titlebar-menu-container {
		position: relative;
		z-index: 20;
		height: 100%;
	}
	.titlebar-menu {
		display: flex;
		align-items: center;
		height: 100%;
	}

	/* ALTERADO: Adicionando Margem e Estilo Aberto */
	.menu-button {
		background: none;
		border: none;
		padding: 0 8px;
		margin: 0 4px;
		font: inherit;
		font-size: 13px;
		color: var(--color-text);
		display: inline-flex;
		align-items: center;
		height: 100%;
		cursor: default;
		/* Efeito de abertura para o botão ativo */
		transition: background 0.1s; 
	}
	.menu-button:hover,
	.menu-button.open {
		background: var(--color-hover);
		border-radius: 4px 4px 0 0; /* Arredonda só em cima quando aberto */
	}
	.menu-button:focus,
	.menu-button:focus-visible {
		outline: none;
		box-shadow: none;
	}
	
	/* Estilo para o Dropdown */
	.menu-dropdown {
		position: absolute;
		top: 100%;
		/* Alinha com o primeiro botão */
		left: 4px; 
		background: var(--color-background);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		min-width: 200px;
		padding: 4px 0;
		z-index: 30; /* Garante que fique acima de outros elementos */
	}

	/* NOVO: Ajusta a posição do menu TEMA */
	.menu-dropdown.theme-menu {
		/* Desloca manualmente a posição Left para que o menu Tema
		   fique posicionado corretamente abaixo de seu botão.
		   4px (margem do botão Arquivo) + 8px (padding Arquivo) + 8px (padding Tema) + 4px (margem Tema)
		   É um valor aproximado, dependendo do tamanho do texto.
		   '60px' funciona bem para a maioria dos casos. */
		left: 60px; 
	}
	
	/* Estilo para os Itens do Menu */
	.menu-item {
		display: block;
		width: calc(100% - 16px);
		background: none;
		border: none;
		padding: 6px 12px;
		margin: 2px 8px;
		font: inherit;
		font-size: 13px;
		color: var(--color-text);
		text-align: left;
		cursor: default;
	}
	.menu-item:hover {
		background: var(--color-hover);
		border-radius: 4px;
	}
	.menu-item:focus,
	.menu-item:focus-visible {
		outline: none;
		box-shadow: none;
		background: var(--color-hover);
	}

	/* NOVO: Estilo para indicar o item de menu ATIVO (tema atual) */
	.menu-item.active {
		font-weight: 600; /* Destaque leve */
		color: #0078d4; /* Cor de destaque (azul padrão) */
		background: transparent; /* Garante que não tenha cor de fundo sem hover */
	}
	.menu-item.active:hover {
		background: var(--color-hover); /* Mantém o hover */
	}

	/* --- Controles da Janela (Direita) --- */
	.titlebar-controls {
		display: flex;
		position: relative;
		z-index: 10;
	}
	.titlebar-button {
		background: none;
		border: none;
		padding: 0;
		margin: 0;
		font: inherit;
		display: inline-flex;
		justify-content: center;
		align-items: center;
		width: 40px;
		height: 30px;
		cursor: default;
	}
	.titlebar-button:focus,
	.titlebar-button:focus-visible {
		outline: none;
		box-shadow: none;
	}
	.titlebar-button svg {
		fill: var(--color-subtle-text);
	}
	.titlebar-button:hover {
		background: var(--color-hover);
	}
	.titlebar-button:hover svg {
		fill: var(--color-text);
	}
	.titlebar-button.close:hover {
		background: #e81123;
	}
	.titlebar-button.close:hover svg {
		fill: white;
	}
</style>
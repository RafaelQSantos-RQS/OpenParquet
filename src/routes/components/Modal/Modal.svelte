<script lang="ts">
	import { fade, scale } from 'svelte/transition';

	interface Props {
		title?: string;
		onclose?: () => void;
		children?: import('svelte').Snippet;
	}

	let { title = '', onclose = () => {}, children }: Props = $props();

	function close(): void {
		onclose();
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			close();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<div 
	class="modal-backdrop" 
	onclick={close}
	onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && close()}
	role="button"
	tabindex="0"
	transition:fade={{ duration: 200 }}
>
	<div 
		class="modal-window" 
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		tabindex="-1"
		transition:scale={{ start: 0.95, duration: 200 }}
		onclick={(e) => e.stopPropagation()}
		onkeydown={(e) => e.stopPropagation()}
	>
		<div class="modal-header">
			<h3 id="modal-title">{title}</h3>
			<button class="close-btn" onclick={close} aria-label="Fechar">
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"/></svg>
			</button>
		</div>
		
		<div class="modal-content">
			{#if children}
				{@render children()}
			{/if}
		</div>
	</div>
</div>

<style src="./Modal.css"></style>

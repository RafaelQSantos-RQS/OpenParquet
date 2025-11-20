<script lang="ts">
	import { fade, scale } from 'svelte/transition';

	export let title: string = '';
	export let onclose: () => void = () => {};

	function close() {
		onclose();
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			close();
		}
	}
</script>

<svelte:window on:keydown={handleKeydown} />

<div 
    class="modal-backdrop" 
    on:click={close}
    on:keydown={(e) => (e.key === 'Enter' || e.key === ' ') && close()}
    role="button"
    tabindex="0"
    transition:fade={{ duration: 200 }}
>
	<div 
        class="modal-window" 
        on:click|stopPropagation
        on:keydown|stopPropagation
        role="dialog"
        aria-modal="true"
        aria-labelledby="modal-title"
        tabindex="-1"
        transition:scale={{ start: 0.95, duration: 200 }}
    >
		<div class="modal-header">
			<h3 id="modal-title">{title}</h3>
			<button class="close-btn" on:click={close} aria-label="Fechar">
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"/></svg>
			</button>
		</div>
		
		<div class="modal-content">
			<slot />
			
			</div>
	</div>
</div>

<style src="./Modal.css"></style>